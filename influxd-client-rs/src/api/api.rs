// todo: api

use std::fmt::{Display, Formatter};
use crate::config::Config;
use crate::request_builder::build_http_get_request_with_basic;
use hyper::{Body};
use hyper::client::conn::SendRequest;
use serde_derive::Deserialize;
use hyper::body::Buf;

pub async fn list_all_authorizations(sender: &mut SendRequest<Body>, config: Config)
                                     -> Result<AuthorizationsRes, Box<dyn std::error::Error>> {
    let path = "/api/v2".to_string();
    let request = build_http_get_request_with_basic(path, config);
    let future = sender.send_request(request);
    let result = future.await;
    match result {
        Err(err) => {
            println!("===============================>> send_request err: {}", err);
            Err(Box::new(err))
        }
        Ok(response) => {
            let body_result = hyper::body::aggregate(response).await;
            match body_result {
                Err(err) => Err(Box::new(err)),
                Ok(body) => {
                    let authorization_res_result = serde_json::from_reader(body.reader());
                    match authorization_res_result {
                        Err(err) => {
                            println!("===============================>> json parse err: {}", err);
                            Err(Box::new(err))
                        }
                        Ok(authorization_res) => Ok(authorization_res),
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthorizationsRes {
    pub links: Link,
    pub authorizations: Vec<Authorizations>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Link {
    pub next: String,
    // todo: fix keyword
    // self: String,
    pub prev: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Authorizations {
    pub status: String,
    pub description: String,
    pub org_id: String,
    pub id: String,
    pub token: String,
    pub user_id: String,
    pub user: String,
    pub org: String,
    // todo: others
}

impl Display for AuthorizationsRes {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("code: {:?}, message: {:?}", self.authorizations, self.links);
        Ok(())
    }
}
