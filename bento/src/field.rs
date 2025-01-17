//! Field management module for the Bento API
//!
//! This module provides functionality for retrieving and creating custom fields
//! in the Bento system.

use crate::{Client, Error, Result};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::instrument;

/// Represents field data returned from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldData {
    /// Unique identifier for the field
    pub id: String,
    /// Type of the data object
    #[serde(rename = "type")]
    pub data_type: String,
    /// Field attributes containing metadata
    pub attributes: FieldAttributes,
}

/// Attributes associated with a field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldAttributes {
    /// Display name of the field
    pub name: String,
    /// Unique key identifier for the field
    pub key: String,
    /// Flag indicating if the field is whitelisted
    pub whitelisted: Option<bool>,
    /// Timestamp when the field was created
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
}

impl Client {
    /// Retrieves all custom fields
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or if the response cannot be parsed
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

    /// Creates a new custom field
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key identifier for the new field
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The key is empty
    /// * The API request fails
    /// * The response cannot be parsed
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
                    "data": [{
                        "id": "field_123",
                        "type": "field",
                        "attributes": {
                            "name": "Test Field",
                            "key": "test_field",
                            "whitelisted": true,
                            "created_at": "2024-01-16T00:00:00Z"
                        }
                    }]
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.get_fields().await;

        assert!(result.is_ok(), "Expected OK, got {:?}", result);
        let fields = result.unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].attributes.name, "Test Field");
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

        assert!(result.is_ok(), "Expected OK, got {:?}", result);
        let field = result.unwrap();
        assert_eq!(field.attributes.name, "Test Field");
        assert_eq!(field.attributes.key, "test_field");
    }

    #[tokio::test]
    async fn test_create_field_validation() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        let result = client.create_field("").await;
        assert!(matches!(result, Err(Error::InvalidRequest(_))));
    }
}