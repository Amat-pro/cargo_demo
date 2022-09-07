use hyper::{Body, Client};
use hyper::client::HttpConnector;

pub fn new_default_http_client() -> Client<HttpConnector, Body> {
    let client = Client::new();
    return client;
}

// todo: sender
// pub fn new_sender_with_block(config: Config) -> Result<SendRequest<Body>, Box<dyn std::error::Error>> {
//     let addr = format!("{}:{}", config.server.ip, config.server.port);
//     let rt = Runtime::new().unwrap();
//     let tcp_stream_result = rt.block_on(TcpStream::connect(addr));
//     match tcp_stream_result {
//         Err(err) => Err(Box::new(err)),
//         Ok(tcp_stream) => {
//             let conn_result = rt.block_on(conn::handshake(tcp_stream));
//             match conn_result {
//                 Err(err) => Err(Box::new(err)),
//                 Ok((sender, conn)) => {
//                     // todo: conn.await
//                     // tokio::task::spawn(async move {
//                     //     if let Err(err) = conn.await {
//                     //         println!("Connection failed: {:?}", err);
//                     //     }
//                     // });
//                     let rt2 = Runtime::new().unwrap();
//                     rt2.block_on(conn).expect("TODO: panic message");
//                     Ok(sender)
//                 }
//             }
//         }
//     }
// }

