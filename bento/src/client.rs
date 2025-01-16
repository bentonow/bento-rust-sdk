use crate::{Config, Error};
use reqwest::{Client as ReqwestClient, RequestBuilder};
use std::time::Duration;
use tracing::{error, instrument};

/// Client for making requests to the Bento API
#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    pub(crate) http_client: ReqwestClient,
}

impl Client {
    /// Create a new client instance
    pub fn new(config: Config) -> crate::Result<Self> {
        let http_client = ReqwestClient::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;

        Ok(Self {
            config,
            http_client,
        })
    }

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

    async fn execute_with_retry(&self, builder: RequestBuilder) -> crate::Result<reqwest::Response> {
        let retry_strategy = tokio_retry::strategy::ExponentialBackoff::from_millis(100)
            .max_delay(Duration::from_secs(5))
            .take(3);

        tokio_retry::RetryIf::spawn(
            retry_strategy,
            {
                let original_builder = builder
                    .try_clone()
                    .ok_or_else(|| Error::InvalidRequest("Failed to clone request".into()))?;

                move || {
                    let builder = original_builder
                        .try_clone()
                        .ok_or_else(|| Error::InvalidRequest("Failed to clone request in retry".into()));

                    async move {
                        let builder = builder?;
                        let response = builder
                            .basic_auth(&self.config.publishable_key, Some(&self.config.secret_key))
                            .header("Accept", "application/json")
                            .header("Content-Type", "application/json")
                            .header(
                                "User-Agent",
                                format!(
                                    "bento-rust-{}-{}",
                                    crate::VERSION,
                                    self.config.site_uuid
                                ),
                            )
                            .send()
                            .await?;

                        Ok(response)
                    }
                }
            },
            |err: &Error| matches!(err, Error::RateLimit),
        ).await
    }

    pub(crate) fn build_url(&self, path: &str) -> crate::Result<String> {
        Ok(format!("{}/{}", self.config.base_url.trim_end_matches('/'), path.trim_start_matches('/')))
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
    async fn test_request_retry() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(429))
            .expect(2)
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})))
            .expect(1)
            .mount(&mock_server)
            .await;

        let config = Config {
            publishable_key: "pub_key".into(),
            secret_key: "secret_key".into(),
            site_uuid: "site_123".into(),
            timeout: Duration::from_secs(30),
            base_url: mock_server.uri(),
        };

        let client = Client::new(config).unwrap();
        let request = client.http_client.get(format!("{}/test", mock_server.uri()));
        let response = client.request(request).await;

        assert!(response.is_ok());
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