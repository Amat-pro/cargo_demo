// @link: https://hyper.rs/guides/client/basic/
// http demo

// const INFLUX_DB_ADDR: &str = "192.168.9.111";
// const INFLUX_DB_PORT: i16 = 8086;
//
// const INFLUX_DB_HTTP_PATH_CONFIG: &str = "/api/v2/config";

use hyper::body::Buf;
use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};
use tokio::runtime::Runtime;
// use std::{thread, time};

fn main() {
    println!("====>>> start");

    // create a tokio runTime (only use tokio runtime)
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(do_get());

    match result {
        Ok(()) => {
            println!("====>> success")
        }
        Err(_err) => {
            println!("====>> error")
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

async fn do_get() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = hyper::Client::new();

    let url = "http://192.168.9.111:8086/api/v2/config";
    let res = client.get(hyper::Uri::from_static(url)).await?;

    if !res.status().is_success() {
        // panic!("==>> statusCode not 200!")
        println!("==> status code not 200: statusCode: {}", res.status());
    }

    println!("===================================================================");

    let body = hyper::body::aggregate(res).await?;
    let config_info: ConfigInfo = serde_json::from_reader(body.reader())?;

    // todo:
    // OUTPUT:
    println!("{}", config_info.clone());

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
