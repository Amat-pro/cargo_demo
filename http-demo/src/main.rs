// @link: https://hyper.rs/guides/client/basic/
// http demo

use hyper::body::Buf;
use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};
use tokio::runtime::Runtime;

fn main() {
    println!("====>>> start");

    println!("====>>> do get use client");
    // create a tokio runTime (only use tokio runtime)
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(do_get_use_client());

    match result {
        Ok(()) => {
            println!("====>> success")
        }
        Err(err) => {
            println!("====>> error: {}", err)
        }
    }


    println!("====>>> do get use sender");
    let result2 = rt.block_on(do_get_use_sender());
    match result2 {
        Ok(()) => {
            println!("====>> success")
        }
        Err(err) => {
            println!("====>> error: {}", err)
        }
    }

    println!("====>>> end");

    // println!("start");
    // let _result = panic_test();
    // println!("end");
    //
    // let ten_secs = time::Duration::from_secs(10);
    // thread::sleep(ten_secs);
}

// panic, will interrupt the program
// fn panic_test() -> Result<(), Box<dyn std::error::Error>> {
//     panic!("test panic")
// }

// ----------------------------------------High-Level-----------------------------------------------
// ===========================================First=================================================
async fn do_get_use_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = hyper::Client::new();

    let url = "http://47.102.103.0:8086/api/v2/config";
    let res = client.get(hyper::Uri::from_static(url)).await?;

    if !res.status().is_success() {
        // panic!("==>> statusCode not 200!")
        println!("==> status code not 200: statusCode: {}", res.status());
    }

    println!("===================================================================");

    let body = hyper::body::aggregate(res).await?;
    let config_info: ConfigInfo = serde_json::from_reader(body.reader())?;

    // OUTPUT:
    println!("OUTPUT: {}", config_info.clone());

    Ok(())
}


#[derive(Deserialize, Debug, Clone)]
struct ConfigInfo {
    code: String,
    message: String,
    // data: String,
}

impl Display for ConfigInfo {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("code: {}, message: {}", self.code, self.message);
        Ok(())
    }
}

// -----------------------------------------Low-Level-----------------------------------------------
// ===========================================Second================================================
use tower::ServiceExt;
use http::{Request, StatusCode};
use hyper::{client::conn, Body};
use tokio::net::TcpStream;


async fn do_get_use_sender() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_stream = TcpStream::connect("example.com:80").await?;

    let (mut request_sender, connection) = conn::handshake(tcp_stream).await?;

    // spawn a task to poll the connection and drive the HTTP state
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("Error in connection: {}", e);
        }
    });
    let request = Request::builder()
        // We need to manually add the host header because SendRequest does not
        .header("Host", "example.com")
        .method("GET")
        .body(Body::from(""))?;
    let response = request_sender.send_request(request).await?;
    println!("=================================== 1 ");
    assert_eq!(response.status(), StatusCode::OK);

    // To send via the same connection again, it may not work as it may not be ready,
    // so we have to wait until the request_sender becomes ready.
    request_sender.ready().await?;

    let request = Request::builder()
        .header("Host", "example.com")
        .method("GET")
        .body(Body::from(""))?;
    let response = request_sender.send_request(request).await?;
    println!("=================================== 2 ");
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}

