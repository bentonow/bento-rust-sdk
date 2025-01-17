use crate::{Client, EventData, EventsRequest, Error, Result};
use serde::Deserialize;
use tracing::instrument;

#[derive(Debug, Deserialize)]
struct EventResponse {
    results: u32,
    failed: u32,
}

impl Client {
    /// Track events for subscribers
    ///
    /// # Arguments
    /// * `events` - Vector of events to track
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    ///
    /// # Errors
    /// * `Error::InvalidRequest` if no events are provided
    /// * `Error::InvalidEmail` if any email is invalid
    /// * `Error::InvalidRequest` if any event type is empty
    /// * `Error::UnexpectedResponse` if the API returns an error
    #[instrument(skip(self))]
    pub async fn track_events(&self, events: Vec<EventData>) -> Result<()> {
        if events.is_empty() {
            return Err(Error::InvalidRequest("No events provided".into()));
        }

        for event in &events {
            if !event.email.contains('@') {
                return Err(Error::InvalidEmail(event.email.clone()));
            }
            if event.event_type.is_empty() {
                return Err(Error::InvalidRequest("Event type is required".into()));
            }
        }

        let url = self.build_url("/batch/events")?;
        let request_data = EventsRequest { events };

        let response = self.request(
            self.http_client
                .post(&url)
                .json(&request_data)
        ).await?;

        let event_response: EventResponse = response.json().await?;

        if event_response.failed > 0 {
            return Err(Error::UnexpectedResponse(
                format!("Event tracking partially failed: {} succeeded, {} failed",
                        event_response.results, event_response.failed)
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, body_json};

    #[tokio::test]
    async fn test_track_events() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/batch/events"))
            .and(body_json(serde_json::json!({
                "events": [{
                    "type": "test_event",
                    "email": "test@example.com"
                }]
            })))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "results": 1,
                    "failed": 0
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());

        let event = EventData {
            event_type: "test_event".into(),
            email: "test@example.com".into(),
            fields: None,
            details: None,
        };

        let result = client.track_events(vec![event]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_track_events_with_fields() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/batch/events"))
            .and(body_json(serde_json::json!({
                "events": [{
                    "type": "test_event",
                    "email": "test@example.com",
                    "fields": {
                        "first_name": "John",
                        "last_name": "Doe"
                    },
                    "details": {
                        "source": "test"
                    }
                }]
            })))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "results": 1,
                    "failed": 0
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());

        let mut fields = HashMap::new();
        fields.insert("first_name".to_string(), serde_json::json!("John"));
        fields.insert("last_name".to_string(), serde_json::json!("Doe"));

        let mut details = HashMap::new();
        details.insert("source".to_string(), serde_json::json!("test"));

        let event = EventData {
            event_type: "test_event".into(),
            email: "test@example.com".into(),
            fields: Some(fields),
            details: Some(details),
        };

        let result = client.track_events(vec![event]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_track_events_validation() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        // Test empty events
        let result = client.track_events(vec![]).await;
        assert!(matches!(result, Err(Error::InvalidRequest(_))));

        // Test invalid email
        let invalid_event = EventData {
            event_type: "test_event".into(),
            email: "invalid-email".into(),
            fields: None,
            details: None,
        };
        let result = client.track_events(vec![invalid_event]).await;
        assert!(matches!(result, Err(Error::InvalidEmail(_))));

        // Test empty event type
        let invalid_event = EventData {
            event_type: "".into(),
            email: "test@example.com".into(),
            fields: None,
            details: None,
        };
        let result = client.track_events(vec![invalid_event]).await;
        assert!(matches!(result, Err(Error::InvalidRequest(_))));
    }

    #[tokio::test]
    async fn test_track_events_partial_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/batch/events"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "results": 1,
                    "failed": 1
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());

        let event = EventData {
            event_type: "test_event".into(),
            email: "test@example.com".into(),
            fields: None,
            details: None,
        };

        let result = client.track_events(vec![event]).await;
        assert!(matches!(result, Err(Error::UnexpectedResponse(_))));
    }
}