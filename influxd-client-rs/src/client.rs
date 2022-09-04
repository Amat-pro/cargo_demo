use hyper::{Body, Client};
use hyper::client::conn::SendRequest;
use hyper::client::HttpConnector;
use tokio::net::TcpStream;
use crate::config::Config;
use tokio::runtime::Runtime;

pub fn new_default_http_client() -> Client<HttpConnector, Body> {
    let client = Client::new();
    return client;
}

pub fn new_sender_with_block(config: Config) -> Result<SendRequest<Body>, Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", config.server.ip, config.server.port);
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(TcpStream::connect(addr));
    match result {
        Err(err) => Err(Box::new(err)),
        Ok(stream) => {
            let conn_result = rt.block_on(hyper::client::conn::handshake(stream));
            match conn_result {
                Err(err) => Err(Box::new(err)),
                Ok((sender, conn)) => {
                    // todo: conn.await
                    // tokio::task::spawn(async move {
                    //     if let Err(err) = conn.await {
                    //         println!("Connection failed: {:?}", err);
                    //     }
                    // });
                    rt.block_on(conn);
                    Ok(sender)
                }
            }
        }
    }
}

