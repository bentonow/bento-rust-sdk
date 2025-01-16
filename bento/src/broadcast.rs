use crate::{BroadcastData, Client, Error, Result};
use serde::Deserialize;
use tracing::instrument;

impl Client {
    /// Get all broadcasts
    #[instrument(skip(self))]
    pub async fn get_broadcasts(&self) -> Result<Vec<BroadcastData>> {
        let url = self.build_url("/fetch/broadcasts")?;
        let response = self.request(
            self.http_client.get(&url)
        ).await?;

        #[derive(Deserialize)]
        struct BroadcastResponse {
            broadcasts: Vec<BroadcastData>,
        }

        let broadcast_response: BroadcastResponse = response.json().await?;
        Ok(broadcast_response.broadcasts)
    }

    /// Create new broadcasts
    #[instrument(skip(self))]
    pub async fn create_broadcasts(&self, broadcasts: Vec<BroadcastData>) -> Result<()> {
        if broadcasts.is_empty() {
            return Err(Error::InvalidRequest("No broadcasts provided".into()));
        }

        for broadcast in &broadcasts {
            if broadcast.name.is_empty() {
                return Err(Error::InvalidRequest("Broadcast name is required".into()));
            }
            if broadcast.subject.is_empty() {
                return Err(Error::InvalidRequest("Subject is required".into()));
            }
            if broadcast.content.is_empty() {
                return Err(Error::InvalidRequest("Content is required".into()));
            }
            if !broadcast.from.email.contains('@') {
                return Err(Error::InvalidEmail(broadcast.from.email.clone()));
            }
            if broadcast.batch_size_per_hour == 0 {
                return Err(Error::InvalidBatchSize("Batch size must be positive".into()));
            }
        }

        let url = self.build_url("/batch/broadcasts")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(&serde_json::json!({
                    "broadcasts": broadcasts
                }))
        ).await?;

        if !response.status().is_success() {
            return Err(Error::UnexpectedResponse(
                format!("Failed to create broadcasts: {}", response.status())
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BroadcastType, ContactData};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_get_broadcasts() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/fetch/broadcasts"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "broadcasts": [
                        {
                            "name": "Test Broadcast",
                            "subject": "Test Subject",
                            "content": "<p>Test Content</p>",
                            "type": "plain",
                            "from": {
                                "email": "test@example.com"
                            },
                            "batch_size_per_hour": 1000
                        }
                    ]
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.get_broadcasts().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_create_broadcasts() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/batch/broadcasts"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());

        let broadcast = BroadcastData {
            name: "Test Broadcast".into(),
            subject: "Test Subject".into(),
            content: "<p>Test Content</p>".into(),
            broadcast_type: BroadcastType::Plain,
            from: ContactData {
                name: Some("Test Sender".into()),
                email: "sender@example.com".into(),
            },
            inclusive_tags: None,
            exclusive_tags: None,
            segment_id: None,
            batch_size_per_hour: 1000,
        };

        let result = client.create_broadcasts(vec![broadcast]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_broadcasts_validation() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        let invalid_broadcast = BroadcastData {
            name: "".into(),
            subject: "Test Subject".into(),
            content: "<p>Test Content</p>".into(),
            broadcast_type: BroadcastType::Plain,
            from: ContactData {
                name: None,
                email: "invalid-email".into(),
            },
            inclusive_tags: None,
            exclusive_tags: None,
            segment_id: None,
            batch_size_per_hour: 0,
        };

        let result = client.create_broadcasts(vec![invalid_broadcast]).await;
        assert!(result.is_err());
    }
}