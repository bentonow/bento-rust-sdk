use crate::{ApiResponse, Client, CreateSubscriberRequest, CreateSubscriberData, Error, ImportSubscriberData, ImportSubscriberResponse, Result, SubscriberData};
use tracing::instrument;

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
        Ok(api_response.data)
    }

    /// Create a new subscriber with just email
    #[instrument(skip(self))]
    pub async fn create_subscriber(&self, email: &str) -> Result<SubscriberData> {
        if !email.contains('@') {
            return Err(Error::InvalidEmail(email.to_string()));
        }

        let url = self.build_url("/fetch/subscribers")?;
        let request = CreateSubscriberRequest {
            subscriber: CreateSubscriberData {
                email: email.to_string(),
            }
        };

        let response = self.request(
            self.http_client
                .post(&url)
                .json(&request)
        ).await?;

        let api_response: ApiResponse<SubscriberData> = response.json().await?;
        Ok(api_response.data)
    }

    /// Import multiple subscribers with full data
    #[instrument(skip(self))]
    pub async fn import_subscribers(&self, subscribers: Vec<ImportSubscriberData>) -> Result<()> {
        if subscribers.is_empty() {
            return Err(Error::InvalidRequest("No subscribers provided".into()));
        }

        for subscriber in &subscribers {
            if !subscriber.email.contains('@') {
                return Err(Error::InvalidEmail(subscriber.email.clone()));
            }
        }

        let url = self.build_url("/batch/subscribers")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(&serde_json::json!({
                    "subscribers": subscribers
                }))
        ).await?;

        let import_response: ImportSubscriberResponse = response.json().await?;

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
    use wiremock::matchers::{method, path, query_param, body_json};
    use serde_json::json;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_find_subscriber() {
        let mock_server = MockServer::start().await;

        let sample_response = json!({
            "data": {
                "id": "611427554",
                "type": "visitors",
                "attributes": {
                    "uuid": "6125f8be-282d-40b7-bd7c-0944d5988955",
                    "email": "test@example.com",
                    "fields": {
                        "fields": {},
                        "first_name": null,
                        "last_name": null,
                        "timestamp": null
                    },
                    "cached_tag_ids": [],
                    "unsubscribed_at": null
                }
            }
        });

        Mock::given(method("GET"))
            .and(path("/fetch/subscribers"))
            .and(query_param("email", "test@example.com"))
            .respond_with(ResponseTemplate::new(200)  // 200 for find subscriber
                .set_body_json(sample_response))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let subscriber = client.find_subscriber("test@example.com").await;

        assert!(subscriber.is_ok());
        let subscriber = subscriber.unwrap();
        assert_eq!(subscriber.attributes.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_create_subscriber() {
        let mock_server = MockServer::start().await;

        let sample_response = json!({
            "data": {
                "id": "611427554",
                "type": "visitors",
                "attributes": {
                    "uuid": "6125f8be-282d-40b7-bd7c-0944d5988955",
                    "email": "test@example.com",
                    "fields": {
                        "fields": {},
                        "first_name": null,
                        "last_name": null,
                        "timestamp": null
                    },
                    "cached_tag_ids": [],
                    "unsubscribed_at": null
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/fetch/subscribers"))
            .and(body_json(json!({
                "subscriber": {
                    "email": "test@example.com"
                }
            })))
            .respond_with(ResponseTemplate::new(201)  // 201 for create subscriber
                .set_body_json(sample_response))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.create_subscriber("test@example.com").await;

        assert!(result.is_ok());
        let subscriber = result.unwrap();
        assert_eq!(subscriber.attributes.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_import_subscribers() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/batch/subscribers"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(json!({
                    "results": 1,
                    "failed": 0
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());

        let mut custom_fields = HashMap::new();
        custom_fields.insert("company".to_string(), json!("Acme Inc"));

        let subscriber = ImportSubscriberData {
            email: "test@example.com".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            tags: Some("lead,mql".to_string()),
            remove_tags: Some("customer".to_string()),
            custom_fields,
        };

        let result = client.import_subscribers(vec![subscriber]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_import_subscribers_partial_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/batch/subscribers"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(json!({
                    "results": 1,
                    "failed": 1
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());

        let subscriber = ImportSubscriberData {
            email: "test@example.com".to_string(),
            first_name: None,
            last_name: None,
            tags: None,
            remove_tags: None,
            custom_fields: HashMap::new(),
        };

        let result = client.import_subscribers(vec![subscriber]).await;
        assert!(matches!(result, Err(Error::UnexpectedResponse(_))));
    }

    #[tokio::test]
    async fn test_invalid_email() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        let result = client.create_subscriber("invalid-email").await;
        assert!(matches!(result, Err(Error::InvalidEmail(_))));

        let subscriber = ImportSubscriberData {
            email: "invalid-email".to_string(),
            first_name: None,
            last_name: None,
            tags: None,
            remove_tags: None,
            custom_fields: HashMap::new(),
        };

        let result = client.import_subscribers(vec![subscriber]).await;
        assert!(matches!(result, Err(Error::InvalidEmail(_))));
    }
}