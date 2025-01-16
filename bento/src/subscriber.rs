use crate::{ApiResponse, Client, Error, Result, SubscriberData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::instrument;

/// Input for creating or updating subscribers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberInput {
    /// Subscriber email
    pub email: String,
    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Tags to add
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Tags to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
    /// Custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, serde_json::Value>>,
}

impl Client {
    /// Find a subscriber by email
    #[instrument(skip(self))]
    pub async fn find_subscriber(&self, email: &str) -> Result<SubscriberData> {
        if !email.contains('@') {
            return Err(Error::InvalidEmail(email.to_string()));
        }

        let url = self.build_url("/fetch/subscribers")?;
        let response = self.request(
            self.http_client
                .get(&url)
                .query(&[("email", email)])
        ).await?;

        let api_response: ApiResponse<SubscriberData> = response.json().await?;

        if api_response.data.id.is_empty() {
            return Err(Error::InvalidRequest(format!("Subscriber not found: {}", email)));
        }

        Ok(api_response.data)
    }

    /// Create a new subscriber
    #[instrument(skip(self))]
    pub async fn create_subscriber(&self, input: SubscriberInput) -> Result<SubscriberData> {
        if !input.email.contains('@') {
            return Err(Error::InvalidEmail(input.email));
        }

        let url = self.build_url("/fetch/subscribers")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(&serde_json::json!({
                    "subscriber": input
                }))
        ).await?;

        let api_response: ApiResponse<SubscriberData> = response.json().await?;
        Ok(api_response.data)
    }

    /// Import multiple subscribers in batch
    #[instrument(skip(self))]
    pub async fn import_subscribers(&self, subscribers: Vec<SubscriberInput>) -> Result<()> {
        if subscribers.is_empty() {
            return Err(Error::InvalidRequest("No subscribers provided".into()));
        }

        for subscriber in &subscribers {
            if !subscriber.email.contains('@') {
                return Err(Error::InvalidEmail(subscriber.email.clone()));
            }
        }

        #[derive(Deserialize)]
        struct ImportResponse {
            results: u32,
            failed: u32,
        }

        let url = self.build_url("/batch/subscribers")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(&serde_json::json!({
                    "subscribers": subscribers
                }))
        ).await?;

        let import_response: ImportResponse = response.json().await?;

        if import_response.failed > 0 {
            return Err(Error::UnexpectedResponse(
                format!("Import partially failed: {} succeeded, {} failed",
                        import_response.results, import_response.failed)
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn test_find_subscriber() {
        let mock_server = MockServer::start().await;

        let sample_response = ApiResponse {
            data: SubscriberData {
                id: "sub_123".into(),
                data_type: "subscriber".into(),
                attributes: Default::default(),
            }
        };

        Mock::given(method("GET"))
            .and(path("/fetch/subscribers"))
            .and(query_param("email", "test@example.com"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(sample_response))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let subscriber = client.find_subscriber("test@example.com").await;

        assert!(subscriber.is_ok());
    }

    // Additional tests...
}