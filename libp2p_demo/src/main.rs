extern crate libp2p;

use libp2p::futures::executor;
use log::info;

mod chat;

fn main1() {

    // export env for RUST_LOG
    env_logger::init();

    let _ = executor::block_on(chat::base_chat::run_base_chat());
    info!("run base chat done!");
}

fn main() {
    // export env for RUST_LOG
    env_logger::init();

    let _ = executor::block_on(chat::gossipsub_chat::run_gossip_sub_chat());
    info!("run gossip sub chat done!");
}
