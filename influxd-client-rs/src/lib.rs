extern crate core;

pub mod api;
pub mod config;
pub mod client;
mod request_builder;

#[cfg(test)]
mod test {
    use hyper::{Body, Request};
    use tokio::runtime::Runtime;
    use crate::client::{new_default_http_client, new_sender_async};
    use crate::config::{BasicAuthentication, Config, Server};
    use crate::request_builder::build_http_get_request_with_basic;

    #[test]
    fn test_list_all_authorizations() {
        let basic =
            BasicAuthentication::new("maomao".to_string(), "maomao123".to_string());
        let server = Server::new("example.com".to_string(), "80".to_string());
        let config = Config::from(basic, server);

        let rt = Runtime::new().unwrap();

        let sender_result = rt.block_on(
            new_sender_async(config.server.ip.clone(), config.server.port.clone()));
        match sender_result {
            Ok(mut sender) => {
                let request_result = Request::builder()
                    // We need to manually add the host header because SendRequest does not
                    .header("Host", "example.com")
                    .method("GET")
                    .body(Body::from(""));

                match request_result {
                    Err(err) => {
                        println!("request_result err: {:?}", err);
                        ()
                    }
                    Ok(request) => {
                        let response_result = rt.block_on(sender.send_request(request));
                        match response_result {
                            Err(err) => {
                                println!("response_result err: {:?}", err);
                                ()
                            }
                            Ok(response) => {
                                println!("response_result Ok, status code is: {:?}", response.status());
                                ()
                            }
                        }
                    }
                }
            }
            Err(err) => {
                println!("sender_result err: {:?}", err);
                ()
            }
        }
    }


    #[test]
    fn test_list_all_authorizations_with_client() {
        let basic =
            BasicAuthentication::new("maomao".to_string(), "maomao123".to_string());
        let server = Server::new("47.102.103.0".to_string(), "8086".to_string());
        let config = Config::from(basic, server);

        let http_client = new_default_http_client();
        let request = build_http_get_request_with_basic(
            "/api/v2".to_string(), config.clone());

        // block for result
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(http_client.request(request));
        match result {
            Err(err) => {
                println!("===>> request fail, err: {}", err);
            }
            Ok(res) => {
                println!("===>> res.status: {}", res.status());
            }
        }
    }
}