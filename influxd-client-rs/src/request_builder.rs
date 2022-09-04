use hyper::{Body, Method, Request};
use crate::config;

pub fn build_http_get_request_with_basic(path: String, config: config::Config) -> Request<Body> {
    let mut path2 = path.clone();
    if !path.starts_with("/") {
        path2 = format!("{}{}", "/", path)
    }
    let url = format!("http://{}:{}{}", config.server.ip, config.server.port, path2);
    // let url = path2;

    let basic_token = format!("{} {}", "Basic", config.basic_authentication.base64());
    let request = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header(hyper::header::AUTHORIZATION, basic_token)
        .body(Body::empty())
        .unwrap();

    return request;
}