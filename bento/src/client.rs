//! Client implementation for making HTTP requests.

use crate::{Config, Error};
use reqwest::{Client as ReqwestClient, RequestBuilder};
use std::time::Duration;
use std::sync::Arc;
use tracing::{error, instrument};
use base64::engine::Engine;

/// Client for making requests to the Bento API.
///
/// This client handles authentication, retry logic, and request/response processing.
#[derive(Debug, Clone)]
pub struct Client {
    config: Arc<Config>,
    pub(crate) http_client: ReqwestClient,
}

impl Client {
    /// Creates a new client instance with the provided configuration.
    ///
    /// # Errors
    /// Returns an error if the HTTP client cannot be created.
    pub fn new(config: Config) -> crate::Result<Self> {
        let http_client = ReqwestClient::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;

        Ok(Self {
            config: Arc::new(config),
            http_client,
        })
    }

    /// Makes an HTTP request with automatic retry handling.
    ///
    /// # Errors
    /// Returns an error if the request fails after retries or receives an error response.
    #[instrument(skip(self))]
    pub(crate) async fn request(&self, builder: RequestBuilder) -> crate::Result<reqwest::Response> {
        let response = self.execute_with_retry(builder).await?;

        match response.status() {
            status if status.is_success() => Ok(response),
            status if status.as_u16() == 429 => Err(Error::RateLimit),
            status if status.as_u16() == 401 => Err(Error::AuthenticationFailed),
            status => {
                let error_msg = response.text().await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!(?status, error = ?error_msg, "API request failed");
                Err(Error::UnexpectedResponse(error_msg))
            }
        }
    }

    /// Executes a request with retry logic for rate limiting.
    async fn execute_with_retry(&self, builder: RequestBuilder) -> crate::Result<reqwest::Response> {
        let retry_strategy = tokio_retry::strategy::ExponentialBackoff::from_millis(100)
            .max_delay(Duration::from_secs(5))
            .take(3);

        let config = Arc::clone(&self.config);
        let original_builder = builder.try_clone()
            .ok_or_else(|| Error::InvalidRequest("Failed to clone request".into()))?;

        tokio_retry::RetryIf::spawn(
            retry_strategy,
            move || {
                let builder = original_builder.try_clone()
                    .ok_or_else(|| Error::InvalidRequest("Failed to clone request".into()));
                let config = Arc::clone(&config);

                async move {
                    let builder = builder?;
                    let response = builder
                        .header("Authorization", format!("Basic {}", base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", config.publishable_key, config.secret_key))))
                        .header("Accept", "application/json")
                        .header("Content-Type", "application/json")
                        .header(
                            "User-Agent",
                            format!(
                                "bento-rust-{}-{}",
                                crate::VERSION,
                                config.site_uuid
                            ),
                        )
                        .send()
                        .await?;

                    Ok(response)
                }
            },
            |err: &Error| matches!(err, Error::RateLimit),
        ).await
    }

    /// Builds a URL by combining the base URL with the provided path.
    ///
    /// # Errors
    /// Returns an error if the URL cannot be constructed.
    pub(crate) fn build_url(&self, path: &str) -> crate::Result<String> {
        let base = format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );

        // Check if the path already contains query parameters
        if base.contains('?') {
            Ok(format!("{}&site_uuid={}", base, self.config.site_uuid))
        } else {
            Ok(format!("{}?site_uuid={}", base, self.config.site_uuid))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_client_creation() {
        let config = Config {
            publishable_key: "pub_key".into(),
            secret_key: "secret_key".into(),
            site_uuid: "site_123".into(),
            timeout: Duration::from_secs(30),
            base_url: "https://api.test.com".into(),
        };

        let client = Client::new(config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_authentication_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(401))
            .expect(1)
            .mount(&mock_server)
            .await;

        let config = Config {
            publishable_key: "invalid".into(),
            secret_key: "invalid".into(),
            site_uuid: "site_123".into(),
            timeout: Duration::from_secs(30),
            base_url: mock_server.uri(),
        };

        let client = Client::new(config).unwrap();
        let request = client.http_client.get(format!("{}/test", mock_server.uri()));
        let response = client.request(request).await;

        assert!(matches!(response, Err(Error::AuthenticationFailed)));
    }
}