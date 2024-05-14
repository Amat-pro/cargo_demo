use tracing::{info, instrument, Level, warn};
use tokio::runtime::Runtime;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::DEBUG)
        // build but do not install the subscriber.
        .finish()
        .init();

    let rt = Runtime::new().unwrap();
    info!("start test ...");
    rt.block_on(test_async(12));
}


#[instrument]
async fn test_async(i: i32) {
    info!("test async do, {}", i);
    warn!("test async end")
}