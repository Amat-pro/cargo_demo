use std::error::Error;

mod chat;

// run this func, and connect to this server use at least two terminal,
// then you can chat like a group chat. Please enjoy you self.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, tokio!");
    chat::chat().await
}
