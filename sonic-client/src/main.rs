use sonic_channel::*;

fn main() {
    let channel_result = IngestChannel::start(
        "192.168.9.111:30909", "sonic123",
    );

    match channel_result {
        Err(e) => {
            println!("error: {}", e)
        }
        Ok(channel) => {
            let dest = Dest::col_buc("collection", "bucket").obj("object:1");
            let pushed = channel.push(PushRequest::new(dest, "my best recipe"));

            match pushed {
                Err(e) => {
                    println!("ingest failed, err: {}", e)
                }
                _ => {
                    println!("ingest success !!!")
                }
            }
        }
    }
}
