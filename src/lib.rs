//! Full-featured crate to interact with Codewars API.
//! Check [Codewars official documentation](https://dev.codewars.com/) for more information about API
//!
//! See [docs.rs](https://docs.rs/codewars-api/latest/codewars_api) and [GitHub repo](https://github.com/ankddev/codewars-api-rs) for more information about this crate

// Warn if something is not documented
#![warn(missing_docs)]

pub mod rest_api;

// Re-exports
pub use crate::rest_api::client::RestCodewarsClient;
pub use crate::rest_api::models::AuthoredChallenges;
pub use crate::rest_api::models::CodeChallenge;
pub use crate::rest_api::models::CompletedChallenges;
pub use crate::rest_api::models::User;
