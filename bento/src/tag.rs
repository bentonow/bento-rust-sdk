use crate::{Client, Error, Result};
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Tag data returned from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagData {
    /// Tag ID
    pub id: String,
    /// Data type
    #[serde(rename = "type")]
    pub data_type: String,
    /// Tag attributes
    pub attributes: TagAttributes,
}

/// Tag attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagAttributes {
    /// Tag name
    pub name: String,
    /// Creation timestamp
    pub created_at: String,
    /// Discard timestamp if tag is discarded
    pub discarded_at: Option<String>,
    /// Site ID
    pub site_id: i32,
}

impl Client {
    /// Get all tags
    #[instrument(skip(self))]
    pub async fn get_tags(&self) -> Result<Vec<TagData>> {
        let url = self.build_url("/fetch/tags")?;
        let response = self.request(
            self.http_client.get(&url)
        ).await?;

        #[derive(Deserialize)]
        struct TagResponse {
            data: Vec<TagData>,
        }

        let tag_response: TagResponse = response.json().await?;
        Ok(tag_response.data)
    }

    /// Create a new tag
    #[instrument(skip(self))]
    pub async fn create_tag(&self, name: &str) -> Result<TagData> {
        if name.is_empty() {
            return Err(Error::InvalidRequest("Tag name is required".into()));
        }

        let url = self.build_url("/fetch/tags")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(&serde_json::json!({
                    "tag": {
                        "name": name
                    }
                }))
        ).await?;

        #[derive(Deserialize)]
        struct TagResponse {
            data: TagData,
        }

        let tag_response: TagResponse = response.json().await?;
        Ok(tag_response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_get_tags() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/fetch/tags"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "data": [
                        {
                            "id": "tag_123",
                            "type": "tag",
                            "attributes": {
                                "name": "test_tag",
                                "created_at": "2024-01-16T00:00:00Z",
                                "site_id": 1
                            }
                        }
                    ]
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.get_tags().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_create_tag() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/fetch/tags"))
            .respond_with(ResponseTemplate::new(201)
                .set_body_json(serde_json::json!({
                    "data": {
                        "id": "tag_123",
                        "type": "tag",
                        "attributes": {
                            "name": "test_tag",
                            "created_at": "2024-01-16T00:00:00Z",
                            "site_id": 1
                        }
                    }
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.create_tag("test_tag").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_tag_validation() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        let result = client.create_tag("").await;
        assert!(matches!(result, Err(Error::InvalidRequest(_))));
    }
}