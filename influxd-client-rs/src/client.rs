use hyper::{Body, Client};
use hyper::client::HttpConnector;

pub fn new_default_http_client() -> Client<HttpConnector, Body> {
    let client = Client::new();
    return client;
}

use hyper::client::conn::SendRequest;
use hyper::client::conn;
use tokio::net::TcpStream;

pub async fn new_sender_async(ip: String, port: String) -> Result<SendRequest<Body>, Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", ip, port);
    let tcp_stream_result = TcpStream::connect(addr).await;
    match tcp_stream_result {
        Err(err) => {
            println!("tcp_stream_result err: {:?}", err);
            Err(Box::new(err))
        }
        Ok(tcp_stream) => {
            let handshake_result = conn::handshake(tcp_stream).await;
            match handshake_result {
                Err(err) => {
                    println!("handshake_result err: {:?}", err);
                    Err(Box::new(err))
                }
                Ok((request_sender, connection)) => {
                    // spawn a task to poll the connection and drive the HTTP state
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            println!("Error in connection: {}", e);
                        }
                    });

                    Ok(request_sender)
                }
            }
        }
    }
}

