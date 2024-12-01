//! Simple example that shows how to read username from stdin and print skills of a user

extern crate codewars_api;
extern crate tokio;

use codewars_api::rest_api::client::RestCodewarsClient;

#[tokio::main]
async fn main() {
    // Initialize client
    let client = RestCodewarsClient::new();
    // Get username from stdin
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    println!("Skills of user:");
    // Get user and print skills
    client
        .get_user(&username.trim())
        .await
        .unwrap()
        .skills
        .iter()
        .for_each(|s| println!("- {}", s));
}
