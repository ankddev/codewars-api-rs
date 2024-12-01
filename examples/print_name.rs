//! Simple example that prints name of user

extern crate codewars_api;
extern crate tokio;

use codewars_api::rest_api::client::RestCodewarsClient;

#[tokio::main]
async fn main() {
    // Initialize client
    let client = RestCodewarsClient::new();
    // Get user and print name
    println!(
        "Name of user: {}",
        client.get_user("ANKDDEV").await.unwrap().name
    );
}
