mod chat;

extern crate libp2p;

use libp2p::futures::executor;
use log::info;

fn main() {

    // export env for RUST_LOG
    env_logger::init();

    let _ = executor::block_on(chat::base_chat::run_base_chat());
    info!("run base chat done!");
}
