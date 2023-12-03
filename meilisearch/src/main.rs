use meilisearch_sdk::client::*;
use serde::{Serialize, Deserialize};
use futures::executor::block_on;

fn main() {
    println!("Hello, meilisearch!");

    block_on(async move {
        // Create a client (without sending any request so that can't fail)
        let client = Client::new("http://localhost:7700", None::<String>);

        // An index is where the documents are stored.
        let movies = client.index("movies");

        // Add some movies in the index. If the index 'movies' does not exist, Meilisearch creates it when you first add the documents.
        movies.add_documents(&[
            Movie { id: 1, title: String::from("Carol"), genres: vec!["Romance".to_string(), "Drama".to_string()] },
            Movie { id: 2, title: String::from("Wonder Woman"), genres: vec!["Action".to_string(), "Adventure".to_string()] },
            Movie { id: 3, title: String::from("Life of Pi"), genres: vec!["Adventure".to_string(), "Drama".to_string()] },
            Movie { id: 4, title: String::from("Mad Max"), genres: vec!["Adventure".to_string(), "Science Fiction".to_string()] },
            Movie { id: 5, title: String::from("Moana"), genres: vec!["Fantasy".to_string(), "Action".to_string()] },
            Movie { id: 6, title: String::from("Philadelphia"), genres: vec!["Drama".to_string()] },
        ], Some("id")).await.unwrap();
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: usize,
    title: String,
    genres: Vec<String>,
}