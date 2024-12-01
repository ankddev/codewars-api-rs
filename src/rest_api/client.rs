//! Client for interacting with the Codewars REST API

use std::string::ToString;
use crate::rest_api::models::{AuthoredChallenges, CodeChallenge, CompletedChallenges, User};

/// Client for interacting with the Codewars API
#[derive(Clone)]
pub struct RestCodewarsClient {
    host_name: String
}

/// Implementation of RestCodewarsClient
impl RestCodewarsClient {
    /// Create new instance of RestCodewarsClient
    ///
    /// # Examples
    ///
    /// ```
    /// use codewars_api::rest_api::client::RestCodewarsClient;
    ///
    /// // We use Tokio here, because all functions of client is asynchronous
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = RestCodewarsClient::new();
    ///     // We can use methods of client here
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            host_name: "https://www.codewars.com".to_string()
        }
    }

    /// Create new instance of RestCodewarsClient with custom host name
    /// Currently this used for unit tests
    #[doc(hidden)]
    pub(crate) fn new_with_custom_host(host_name: String) -> Self {
        Self {
            host_name
        }
    }

    /// Get info about user by username
    ///
    /// # Arguments:
    /// * username (&str) - username of the user
    ///
    /// # Returns:
    /// * Result<User, String> - Result that contains the user or an error message
    ///
    /// # Errors:
    /// * `unexpected status code: {status_code}` - If the status code is not 200
    /// * `error decoding response body` - If there is an error decoding the response body with serde
    ///
    /// # Examples
    /// ```no_run
    /// # use codewars_api::rest_api::client::RestCodewarsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = RestCodewarsClient::new();
    /// let user = client.get_user("ANKDDEV").await.unwrap();
    /// // Get name of user
    /// println!("Name: {}", user.name);
    /// // Get leaderboard position of user
    /// println!("Leaderboard position: {}", user.leaderboard_position);
    /// # }
    /// ```
    pub async fn get_user(&self, username: &str) -> Result<User, String> {
        // Send request
        let response = reqwest::get(format!(
            "{}/api/v1/users/{}",
            self.host_name, username
        ))
        .await
        .unwrap();
        // Check status code
        match response.status() {
            reqwest::StatusCode::OK => match response.json::<User>().await {
                // Return parsed response
                Ok(parsed) => Ok(parsed),
                // Return error if there is an error decoding the response body with serde
                Err(err) => Err(err.to_string()),
            },
            // Return error if status code is not 200
            other => Err(format!("unexpected status code: {}", other)),
        }
    }

    /// Get info about kata by slug
    ///
    /// # Arguments:
    /// * slug (&str) - slug of the kata
    ///
    /// # Returns:
    /// * Result<CodeChallenge, String> - Result that contains the kata or an error message
    ///
    /// # Errors:
    /// * `unexpected status code: {status_code}` - If the status code is not 200
    /// * `error decoding response body` - If there is an error decoding the response body with serde
    ///
    /// # Examples
    /// ```no_run
    /// # use codewars_api::rest_api::client::RestCodewarsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = RestCodewarsClient::new();
    /// let kata = client.get_kata("576bb71bbbcf0951d5000044").await.unwrap();
    /// // Get name of code challenge
    /// println!("Name: {}", kata.name);
    /// // Get slug of code challenge
    /// println!("Slug: {}", kata.slug);
    /// # }
    /// ```
    pub async fn get_kata(&self, slug: &str) -> Result<CodeChallenge, String> {
        // Send request
        let response = reqwest::get(format!(
            "{}/api/v1/code-challenges/{}",
            self.host_name, slug
        ))
        .await
        .unwrap();
        // Check status code
        match response.status() {
            reqwest::StatusCode::OK => match response.json::<CodeChallenge>().await {
                // Return parsed response
                Ok(parsed) => Ok(parsed),
                // Return error if there is an error decoding the response body with serde
                Err(err) => Err(err.to_string()),
            },
            // Return error if status code is not 200
            other => Err(format!("unexpected status code: {}", other)),
        }
    }

    /// Get list of completed challenges
    ///
    /// # Arguments:
    /// * username (&str) - username of the user
    /// * page (u16) - page number
    ///
    /// # Returns:
    /// * Result<CompletedChallenges, String> - Result that contains the list of completed challenges or an error message
    ///
    /// # Errors:
    /// * `unexpected status code: {status_code}` - If the status code is not 200
    /// * `error decoding response body` - If there is an error decoding the response body with serde///
    ///
    /// # Examples
    /// ```no_run
    /// # use codewars_api::rest_api::client::RestCodewarsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = RestCodewarsClient::new();
    /// let challenges = client.get_completed_challenges("ANKDDEV", 0).await.unwrap();
    /// // Get total number of pages
    /// println!("Total pages: {}", challenges.total_pages);
    /// // Get total number of items
    /// println!("Total items: {}", challenges.total_items);
    /// # }
    /// ```
    pub async fn get_completed_challenges(
        &self,
        username: &str,
        page: u16,
    ) -> Result<CompletedChallenges, String> {
        // Send request
        let response = reqwest::get(format!(
            "{}/api/v1/users/{}/code-challenges/completed?page={}",
            self.host_name, username, page
        ))
        .await
        .unwrap();
        // Check status code
        match response.status() {
            reqwest::StatusCode::OK => match response.json::<CompletedChallenges>().await {
                // Return parsed response
                Ok(parsed) => Ok(parsed),
                // Return error if there is an error decoding the response body with serde
                Err(err) => Err(err.to_string()),
            },
            // Return error if status code is not 200
            other => Err(format!("unexpected status code: {}", other)),
        }
    }

    /// Get first page of completed challenges
    ///
    /// # Arguments:
    /// * username (&str) - username of the user
    ///
    /// # Returns:
    /// * Result<CompletedChallenges, String> - Result that contains the list of completed challenges or an error message
    ///
    /// # Errors:
    /// * `unexpected status code: {status_code}` - If the status code is not 200
    /// * `error decoding response body` - If there is an error decoding the response body with serde
    ///
    /// # Examples
    /// ```no_run
    /// # use codewars_api::rest_api::client::RestCodewarsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = RestCodewarsClient::new();
    /// let challenges = client.get_completed_challenges_first_page("ANKDDEV").await.unwrap();
    /// // Get total number of pages
    /// println!("Total pages: {}", challenges.total_pages);
    /// // Get total number of items
    /// println!("Total items: {}", challenges.total_items);
    /// # }
    /// ```
    pub async fn get_completed_challenges_first_page(
        &self,
        username: &str,
    ) -> Result<CompletedChallenges, String> {
        // Return first page of list
        self.get_completed_challenges(username, 0).await
    }

    /// Get list of authored challenges
    ///
    /// # Arguments:
    /// * username (&str) - username of the user
    ///
    /// # Returns:
    /// * Result<AuthoredChallenges, String> - Result that contains the list of authored challenges or an error message
    ///
    /// # Errors:
    /// * `unexpected status code: {status_code}` - If the status code is not 200
    /// * `error decoding response body` - If there is an error decoding the response body with serde
    ///
    /// # Examples
    /// ```no_run
    /// # use codewars_api::rest_api::client::RestCodewarsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = RestCodewarsClient::new();
    /// let challenges = client.get_authored_challenges("Dentzil").await.unwrap();
    /// // Get name of first challenge
    /// println!("Total pages: {}", challenges.data.first().unwrap().name);
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # use codewars_api::rest_api::client::RestCodewarsClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let client = RestCodewarsClient::new();
    /// let challenges = client.get_authored_challenges("aaron.pp").await.unwrap();
    /// // Check if list is not empty
    /// assert!(!challenges.data.is_empty());
    /// # }
    /// ```
    pub async fn get_authored_challenges(
        &self,
        username: &str,
    ) -> Result<AuthoredChallenges, String> {
        // Send request
        let response = reqwest::get(format!(
            "{}/api/v1/users/{}/code-challenges/authored",
            self.host_name, username
        ))
        .await
        .unwrap();
        // Check status code
        match response.status() {
            reqwest::StatusCode::OK => match response.json::<AuthoredChallenges>().await {
                // Return parsed response
                Ok(parsed) => Ok(parsed),
                // Return error if there is an error decoding the response body with serde
                Err(err) => Err(err.to_string()),
            },
            // Return error if status code is not 200
            other => Err(format!("unexpected status code: {}", other)),
        }
    }
}

/// Implement Default trait for RestCodewarsClient
impl Default for RestCodewarsClient {
    // Return default value of RestCodewarsClient
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    //! Tests for REST Client
    //! All mocks are from Codewars documentation

    use std::path::Path;
    use super::{super::models::*, *};

    /// Test getting a user
    #[tokio::test]
    async fn test_get_user() {
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let client = RestCodewarsClient::new_with_custom_host(format!("http://{}", host));
        let content = std::fs::read_to_string(Path::new(&"tests/mocks/get_user.json")).unwrap();
        let text: User = serde_json::from_str(&content).unwrap();
        let mock = server.mock("GET", "/api/v1/users/some_user").with_status(200).with_header("content-type", "application/json").with_body(content).create_async().await;
        let result = client.get_user("some_user").await.unwrap();
        mock.assert_async().await;
        assert_eq!(result, text);
    }

    /// Test getting completed challenges
    #[tokio::test]
    async fn test_get_completed_challenges() {
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let client = RestCodewarsClient::new_with_custom_host(format!("http://{}", host));
        let content = std::fs::read_to_string(Path::new(&"tests/mocks/get_completed_challenges.json")).unwrap();
        let text: CompletedChallenges = serde_json::from_str(&content).unwrap();
        let mock = server.mock("GET", "/api/v1/users/some_user/code-challenges/completed?page=0").with_status(200).with_header("content-type", "application/json").with_body(content).create_async().await;
        let result = client.get_completed_challenges("some_user", 0).await.unwrap();
        mock.assert_async().await;
        assert_eq!(result, text);
    }

    /// Test getting first page of completed challenges
    #[tokio::test]
    async fn test_get_completed_challenges_first_page() {
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let client = RestCodewarsClient::new_with_custom_host(format!("http://{}", host));
        let content = std::fs::read_to_string(Path::new(&"tests/mocks/get_completed_challenges.json")).unwrap();
        let text: CompletedChallenges = serde_json::from_str(&content).unwrap();
        let mock = server.mock("GET", "/api/v1/users/some_user/code-challenges/completed?page=0").with_status(200).with_header("content-type", "application/json").with_body(content).create_async().await;
        let result = client.get_completed_challenges_first_page("some_user").await.unwrap();
        mock.assert_async().await;
        assert_eq!(result, text);
    }

    /// Test getting authored challenges
    #[tokio::test]
    async fn test_get_authored_challenges() {
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let client = RestCodewarsClient::new_with_custom_host(format!("http://{}", host));
        let content = std::fs::read_to_string(Path::new(&"tests/mocks/get_authored_challenges.json")).unwrap();
        let text: AuthoredChallenges = serde_json::from_str(&content).unwrap();
        let mock = server.mock("GET", "/api/v1/users/some_user/code-challenges/authored").with_status(200).with_header("content-type", "application/json").with_body(content).create_async().await;
        let result = client.get_authored_challenges("some_user").await.unwrap();
        mock.assert_async().await;
        assert_eq!(result, text);
    }

    /// Test getting code challenge information
    #[tokio::test]
    async fn test_get_code_challenge() {
        let mut server = mockito::Server::new_async().await;
        let host = server.host_with_port();
        let client = RestCodewarsClient::new_with_custom_host(format!("http://{}", host));
        let content = std::fs::read_to_string(Path::new(&"tests/mocks/get_challenge.json")).unwrap();
        let text: CodeChallenge = serde_json::from_str(&content).unwrap();
        let mock = server.mock("GET", format!("/api/v1/code-challenges/{}", text.slug).as_str()).with_status(200).with_header("content-type", "application/json").with_body(content).create_async().await;
        let result = client.get_kata(&text.slug).await.unwrap();
        mock.assert_async().await;
        assert_eq!(result, text);
    }
}
