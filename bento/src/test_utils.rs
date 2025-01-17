use crate::{Client, Config};
use wiremock::{MockServer};
use std::time::Duration;

/// Utility functions for testing
    use super::*;

    /// Creates a test client with a mock server
    pub fn create_test_client(base_url: String) -> Client {
        let config = Config {
            publishable_key: "test_pub_key".into(),
            secret_key: "test_secret_key".into(),
            site_uuid: "test_site_uuid".into(),
            timeout: Duration::from_secs(30),
            base_url,
        };

        Client::new(config).expect("Failed to create test client")
    }

    /// Starts a mock server and returns the instance
    pub async fn start_mock_server() -> MockServer {
        MockServer::start().await
    }


#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_create_test_client() {
        let mock_server = start_mock_server().await;
        let client = create_test_client(mock_server.uri());

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "status": "ok" })))
            .mount(&mock_server)
            .await;

        let request = client.http_client.get(format!("{}/test", mock_server.uri()));
        let response = client.request(request).await;

        assert!(response.is_ok());
    }
}
