<h1 align="center">codewars-api-rs</h1>
<h6 align="center">Full-featured crate to interact with Codewars API. Check <a href="https://dev.codewars.com/">official documentation</a> for more information about API.</h6>
<div align="center">
    <a href="https://crates.io/crates/codewars-api"><img src="https://img.shields.io/crates/d/codewars-api" alt="crates.io downloads"/></a>
    <a href="https://crates.io/crates/codewars-api"><img src="https://img.shields.io/crates/v/codewars-api"/></a>
    <a href="https://docs.rs/codewars-api/latest/codewars_api"><img src="https://docs.rs/codewars-api/badge.svg"/></a>
    <img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/w/ankddev/codewars-api-rs">
    <a href="https://github.com/ankddev/codewars-api-rs/actions/workflows/test.yml"><img src="https://github.com/ankddev/codewars-api-rs/actions/workflows/test.yml/badge.svg" alt="Test status"/></a>
</div>

> [!NOTE]
> At this moment, Codewars API is [minimal and inconsistent](https://dev.codewars.com/#introduction).
> So, you can't to do some things with API and this crate

# Features
- [x] Interact with the Codewars REST API
    - [x] Get user info
    - [x] Get list of completed challenges
    - [x] Get list of authored challenges
    - [x] Get kata info
- [ ] Interact with the Codewars API using webhooks
# Installing
You can install this crate from Crates.io using Cargo:
```shell
$ cargo add codewars-api
```
# Usage
At this moment only REST API is supported, webhook support will be added in the future.
Import it in your project:
```rust
use codewars_api::rest_api::client::RestCodewarsClient;
```
Then, initialize the client:
```rust
let client = RestCodewarsClient::new();
```
And you can use methods of client:
```rust
let user = client.get_user("username").await.unwrap();
let challenges = client.get_completed_challenges("username", 1).await.unwrap();
```
> [!NOTE]
> If you want to use it in `main` function you should install `tokio`
> ```shell
> $ cargo add tokio
> ```
> And then you can use it in `main` function:
> ```rust
> use tokio;
> use codewars_api::rest_api::client::RestCodewarsClient;
>
> #[tokio::main]
> async fn main() {
>     let client = RestCodewarsClient::new();
>     let user = client.get_user("username").await.unwrap();
>     let challenges = client.get_completed_challenges("username", 1).await.unwrap();
> }
> ```
# Documentation
Documentation for this crate can be found at [docs.rs](https://docs.rs/codewars-api/latest/codewars_api)
Also, you can see examples of using this crate in [examples](./examples). To run example clone this repo and run this:
```shell
$ cargo run --example <example_name>
```
For example:
```shell
$ cargo run --example print_name
```
# Contributing
Firstly, you should install [Git](https://git-scm.com/download) and [Rust](https://www.rust-lang.org/tools/install).
* Create fork of this repo by pressing button on the top of this page.
* Clone your fork
```shell
$ git clone https://github.com/username/codewars-api-rs.git
```
* Go to directory where you cloned repo
```shell
$ cd codewars-api-rs
```
* Create new branch
* Make your changes
* Run tests:
```shell
$ cargo test
```
* [Write tests and documentation for your changes](#writing-tests)
* Format and lint code:
```shell
$ cargo fmt
$ cargo clippy
```
* Commit changes
* Create PR
## Writing tests
You can learn more about writing tests in the [official documentation](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).
We test all methods of `RestCodewarsClient` struct in [doctests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html).
At this moment, we don't use mock in doctests, so you should add `no_run` attribute for doctests, like this:

    # Examples
    ```no_run
    use codewars_api::rest_api::client::RestCodewarsClient;
    ...
    ```

In unit tests we use `mockito` library for mock testing. See [it's official documentation](https://docs.rs/mockito/latest/mockito/) for more information. Mocks are stored in `tests/mocks` directory. All mocks are from Codewars documentation.
