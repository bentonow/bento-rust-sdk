use crate::{Client, Error, Result};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::instrument;

/// Field data returned from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldData {
    /// Field ID
    pub id: String,
    /// Data type
    #[serde(rename = "type")]
    pub data_type: String,
    /// Field attributes
    pub attributes: FieldAttributes,
}

/// Field attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldAttributes {
    /// Field name
    pub name: String,
    /// Field key
    pub key: String,
    /// Whether the field is whitelisted
    pub whitelisted: Option<bool>,
    /// Creation timestamp
    pub created_at: OffsetDateTime,
}

impl Client {
    /// Get all custom fields
    #[instrument(skip(self))]
    pub async fn get_fields(&self) -> Result<Vec<FieldData>> {
        let url = self.build_url("/fetch/fields")?;
        let response = self.request(
            self.http_client.get(&url)
        ).await?;

        #[derive(Deserialize)]
        struct FieldResponse {
            data: Vec<FieldData>,
        }

        let field_response: FieldResponse = response.json().await?;
        Ok(field_response.data)
    }

    /// Create a new custom field
    #[instrument(skip(self))]
    pub async fn create_field(&self, key: &str) -> Result<FieldData> {
        if key.is_empty() {
            return Err(Error::InvalidRequest("Field key is required".into()));
        }

        let url = self.build_url("/fetch/fields")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(&serde_json::json!({
                    "field": {
                        "key": key
                    }
                }))
        ).await?;

        #[derive(Deserialize)]
        struct FieldResponse {
            data: FieldData,
        }

        let field_response: FieldResponse = response.json().await?;
        Ok(field_response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_get_fields() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/fetch/fields"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "data": [
                        {
                            "id": "field_123",
                            "type": "field",
                            "attributes": {
                                "name": "Test Field",
                                "key": "test_field",
                                "whitelisted": true,
                                "created_at": "2024-01-16T00:00:00Z"
                            }
                        }
                    ]
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.get_fields().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_create_field() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/fetch/fields"))
            .respond_with(ResponseTemplate::new(201)
                .set_body_json(serde_json::json!({
                    "data": {
                        "id": "field_123",
                        "type": "field",
                        "attributes": {
                            "name": "Test Field",
                            "key": "test_field",
                            "whitelisted": true,
                            "created_at": "2024-01-16T00:00:00Z"
                        }
                    }
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.create_field("test_field").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_field_validation() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        let result = client.create_field("").await;
        assert!(matches!(result, Err(Error::InvalidRequest(_))));
    }
}