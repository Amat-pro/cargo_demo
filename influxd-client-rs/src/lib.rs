extern crate core;

pub mod api;
pub mod config;
pub mod client;
mod request_builder;

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;
    use crate::api::list_all_authorizations;
    use crate::client::{new_default_http_client, new_sender_with_block};
    use crate::config::{BasicAuthentication, Config, Server};
    use crate::request_builder::build_http_get_request_with_basic;

    // todo: remove
    // #[test]
    // fn test_list_all_authorizations() {
    //     let basic =
    //         BasicAuthentication::new("maomao".to_string(), "maomao123".to_string());
    //     let server = Server::new("example.com".to_string(), "80".to_string());
    //     let config = Config::from(basic, server);
    //
    //     let sender_result = new_sender_with_block(config.clone());
    //     match sender_result {
    //         Ok(mut sender) => {
    //             let s = &mut sender;
    //             let rt = Runtime::new().unwrap();
    //             let result =
    //                 rt.block_on(list_all_authorizations(s, config.clone()));
    //             match result {
    //                 Ok(authorization_res) => {
    //                     println!("============================");
    //                     println!("==> Ok, res: {}", authorization_res);
    //                 }
    //                 Err(err) => {
    //                     println!("block on list_all_authorizations err: {}", err);
    //                     ()
    //                 }
    //             }
    //         }
    //         _ => {
    //             println!("new_sender_with_block err");
    //             ()
    //         }
    //     }
    // }


    #[test]
    fn test_list_all_authorizations_with_client() {
        let basic =
            BasicAuthentication::new("maomao".to_string(), "maomao123".to_string());
        let server = Server::new("47.102.103.0".to_string(), "8086".to_string());
        let config = Config::from(basic, server);

        let http_client = new_default_http_client();
        let request = build_http_get_request_with_basic("/api/v2/authrizations".to_string(), config.clone());

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