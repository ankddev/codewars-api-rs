//! Simple example that shows how to print name of first authored challenge

extern crate codewars_api;
extern crate tokio;

use codewars_api::rest_api::client::RestCodewarsClient;

#[tokio::main]
async fn main() {
    // Initialize client
    let client = RestCodewarsClient::new();
    println!(
        "Name of first authored challenge: '{}'",
        client
            .get_authored_challenges("Dentzil")
            .await
            .unwrap()
            .data
            .first()
            .unwrap()
            .name
    );
}
