use std::error::Error;
use async_std::io;
use libp2p::{floodsub, identity, Multiaddr, NetworkBehaviour, PeerId, Swarm};
use libp2p::floodsub::{Floodsub, FloodsubEvent};
use libp2p::futures::{
    prelude::{stream::StreamExt, *},
    select,
};
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};
use libp2p::swarm::SwarmEvent;

use log::info;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
pub struct MyBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,

    #[behaviour(ignore)]
    #[allow(dead_code)]
    ignored_member: bool,
}

#[derive(Debug)]
pub enum OutEvent {
    Floodsub(FloodsubEvent),
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for OutEvent {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

impl From<FloodsubEvent> for OutEvent {
    fn from(v: FloodsubEvent) -> Self {
        Self::Floodsub(v)
    }
}

pub async fn run_base_chat() -> Result<(), Box<dyn Error>> {
    // create a random peerId
    let key_pair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(key_pair.public());
    info!("after create a random peerId: {:?}", peer_id);

    // create a Floodsub topic
    let floodsub_topic = floodsub::Topic::new("chat");
    info!("after create a floodsub topic: chat");

    // set up a tcp transport
    let transport = libp2p::development_transport(key_pair).await?;
    // executor::block_on(transport);
    info!("after execute transport create");

    // create a swarm to manage peers and events
    let mut swarm = {
        let mdns = Mdns::new(MdnsConfig::default()).await?;
        let mut behaviour = MyBehaviour {
            floodsub: Floodsub::new(peer_id),
            mdns,
            ignored_member: false,
        };

        // subscribe the chat topic
        behaviour.floodsub.subscribe(floodsub_topic.clone());
        Swarm::new(transport, behaviour, peer_id)
    };

    // reach out to another node if specified
    if let Some(to_dial) = std::env::args().nth(1) {
        let addr: Multiaddr = to_dial.parse()?;
        swarm.dial(addr)?;
        info!("Dialed {:?}", to_dial);
    }

    // read full lines from stdin
    let buf_reader = io::BufReader::new(io::stdin());
    let lines = buf_reader.lines();
    let mut stdin = lines.fuse();

    // listen on all interfaces and whatever port the os assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => swarm
                .behaviour_mut()
                .floodsub
                .publish(floodsub_topic.clone(), line.expect("Stdin not to close").as_bytes()),
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(OutEvent::Floodsub(
                    FloodsubEvent::Message(message)
                )) => {
                    info!(
                        "Received: '{:?}' from {:?}",
                        String::from_utf8_lossy(&message.data),
                        message.source
                    );
                }
                SwarmEvent::Behaviour(OutEvent::Mdns(
                    MdnsEvent::Discovered(list)
                )) => {
                    for (peer, _) in list {
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .add_node_to_partial_view(peer);
                    }
                }
                SwarmEvent::Behaviour(OutEvent::Mdns(MdnsEvent::Expired(
                    list
                ))) => {
                    for (peer, _) in list {
                        if !swarm.behaviour_mut().mdns.has_node(&peer) {
                            swarm
                                .behaviour_mut()
                                .floodsub
                                .remove_node_from_partial_view(&peer);
                        }
                    }
                },
                _ => {}
            }
        }
    }
}























