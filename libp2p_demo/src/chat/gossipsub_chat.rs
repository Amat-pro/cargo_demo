use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use libp2p::{gossipsub, identity, PeerId, swarm::SwarmEvent};
use libp2p::futures::{AsyncBufReadExt, select, StreamExt};
use libp2p::gossipsub::{GossipsubMessage, GossipsubEvent, MessageAuthenticity, MessageId, ValidationMode};
use log::{info, warn};

use async_std::io;
use async_std::io::{BufReader};

pub async fn run_gossip_sub_chat() -> Result<(), Box<dyn Error>> {
    // create a peerId
    let key_pair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(key_pair.public());
    info!("gossip sub, peerId is: {:?}", peer_id);

    // set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::development_transport(key_pair.clone()).await?;

    // create a topic
    let topic = gossipsub::IdentTopic::new("test-gossip");

    // create a Swarm to manage peers and events
    let mut swarm = {
        // to content-address message, we can take the hash of message and use it as an ID
        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string())
        };

        // set custom gossip sub
        let gossip_sub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .expect("Valid Config");
        // build a gossip sub network behaviour
        let mut gossipsub: gossipsub::Gossipsub =
            gossipsub::Gossipsub::new(MessageAuthenticity::Signed(key_pair), gossip_sub_config)
                .expect("Correct configuration");

        // subscribes to our topic
        gossipsub.subscribe(&topic).unwrap();

        // add an explicit peer id one was provided
        if let Some(explicit) = std::env::args().nth(2) {
            let explicit = explicit.clone();
            match explicit.parse() {
                Ok(id) => gossipsub.add_explicit_peer(&id),
                Err(err) => warn!("Failed to parse explicit peer id: {:?}", err),
            }
        }

        // build a swarm
        libp2p::Swarm::new(transport, gossipsub, peer_id)
    };

    // listener on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // Reach out to another node if specified
    if let Some(to_dial) = std::env::args().nth(1) {
        let address: libp2p::Multiaddr = to_dial.parse().expect("User to provide valid address.");
        match swarm.dial(address.clone()) {
            Ok(_) => println!("Dialed {:?}", address),
            Err(e) => println!("Dial {:?} failed: {:?}", address, e),
        };
    }

    // read full lines from stdin
    let buf_reader = BufReader::new(io::stdin());
    let lines = buf_reader.lines();
    let mut stdin = lines.fuse();

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => {
                let result = swarm.behaviour_mut()
                    .publish(topic.clone(), line.expect("Stdin not to close").as_bytes());
                match result {
                    Ok(_) => {}
                    Err(err) => {
                        warn!("Publish error: {:?}", err);
                    }
                }
            },
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(GossipsubEvent::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                }) => info!(
                    "Got message: {} with id: {} from peer: {:?}",
                    String::from_utf8_lossy(&message.data),
                    id,
                    peer_id
                ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Listening on {:?}", address);
                }
                _ => {}
            }
        }
    }
}























