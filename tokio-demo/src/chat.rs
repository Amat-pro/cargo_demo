use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::io;
use std::env;
use tokio::net::{TcpStream, TcpListener};
use tokio_stream::StreamExt;
use futures::SinkExt;
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec};
use std::sync::Arc;

pub async fn chat() -> Result<(), Box<dyn Error>> {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

    // init tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("chat=trace".parse()?))
        .with_span_events(FmtSpan::FULL)
        .init();

    let state = Arc::new(Mutex::new(SenderManger::new()));
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:6142".to_string());
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        // Spawn our handler to be run asynchronously.
        tokio::spawn(async move {
            tracing::debug!("accepted connection");
            if let Err(e) = process(state, stream, addr).await {
                tracing::info!("an error occurred; error = {:?}", e);
            }
        });
    }
}

type Sender = mpsc::UnboundedSender<String>;

type Receiver = mpsc::UnboundedReceiver<String>;

struct SenderManger {
    peers: HashMap<SocketAddr, Sender>,
}

impl SenderManger {
    fn new() -> Self {
        SenderManger {
            peers: HashMap::new(),

        }
    }

    // send msg to all except self
    async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        for peer in self.peers.iter_mut() {
            if *peer.0 != sender {
                let _ = peer.1.send(message.into());
            }
        }
    }
}

struct Peer {
    lines: Framed<TcpStream, LinesCodec>,
    receiver: Receiver,
}

impl Peer {
    async fn new(
        state: Arc<Mutex<SenderManger>>,
        lines: Framed<TcpStream, LinesCodec>,
    ) -> io::Result<Peer> {
        let addr = lines.get_ref().peer_addr()?;

        // Create a channel for this peer
        let (sender, receiver) = mpsc::unbounded_channel();

        // Add an entry for this `Peer` in the shared state map.
        state.lock().await.peers.insert(addr, sender);

        Ok(Peer { lines, receiver })
    }
}

async fn process(
    state: Arc<Mutex<SenderManger>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    lines.send("Please enter your username:").await?;

    let username = match lines.next().await {
        Some(Ok(line)) => {
            tracing::info!("username: {}", line);
            line
        }
        _ => {
            tracing::error!("Failed to get username from {}. Client disconnected.", addr);
            return Ok(());
        }
    };

    // register peer
    let mut peer = Peer::new(state.clone(), lines).await?;

    // let every one know a new client has connected
    {
        let mut state = state.lock().await;
        let msg = format!("{} has join the chat", username);
        state.broadcast(addr, &msg).await;
    }

    // loop for msg
    loop {
        tokio::select! {
            Some(msg) = peer.receiver.recv() => {
                peer.lines.send(&msg).await?;
            }
            result = peer.lines.next() => match result {
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    let msg = format!("{}: {}", username, msg);

                    state.broadcast(addr, &msg).await;
                },
                Some(Err(e)) => {
                     tracing::error!(
                        "an error occurred while processing messages for {}; error = {:?}",
                        username,
                        e
                    );
                },
                None => break,
            }
        }
    }

    // remove the peer
    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);

        let msg = format!("{} has left the chat", username);
        tracing::info!("{}", msg);
        state.broadcast(addr, &msg).await;
    }

    Ok(())
}


