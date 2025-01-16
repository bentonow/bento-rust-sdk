use crate::{Client, EventData, Error, Result};
use serde::Deserialize;
use tracing::instrument;

impl Client {
    /// Track events for subscribers
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

        #[derive(Deserialize)]
        struct EventResponse {
            results: u32,
            failed: u32,
        }

        let url = self.build_url("/batch/events")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(&serde_json::json!({
                    "events": events
                }))
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
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_track_events() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/batch/events"))
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
            fields: Some(HashMap::new()),
            details: Some(HashMap::new()),
        };

        let result = client.track_events(vec![event]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_track_events_validation() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        let invalid_event = EventData {
            event_type: "".into(),
            email: "invalid-email".into(),
            fields: None,
            details: None,
        };

        let result = client.track_events(vec![invalid_event]).await;
        assert!(result.is_err());
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