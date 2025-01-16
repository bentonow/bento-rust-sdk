use crate::{Client, Error, Result};
use tracing::instrument;

impl Client {
    /// Get site-wide statistics
    #[instrument(skip(self))]
    pub async fn get_site_stats(&self) -> Result<serde_json::Value> {
        let url = self.build_url("/stats/site")?;
        let response = self.request(
            self.http_client.get(&url)
        ).await?;

        let stats = response.json().await?;
        Ok(stats)
    }

    /// Get statistics for a specific segment
    #[instrument(skip(self))]
    pub async fn get_segment_stats(&self, segment_id: &str) -> Result<serde_json::Value> {
        if segment_id.is_empty() {
            return Err(Error::InvalidSegmentId("Segment ID is required".into()));
        }

        let url = self.build_url("/stats/segment")?;
        let response = self.request(
            self.http_client
                .get(&url)
                .query(&[("segment_id", segment_id)])
        ).await?;

        let stats = response.json().await?;
        Ok(stats)
    }

    /// Get statistics for a specific report
    #[instrument(skip(self))]
    pub async fn get_report_stats(&self, report_id: &str) -> Result<serde_json::Value> {
        if report_id.is_empty() {
            return Err(Error::InvalidRequest("Report ID is required".into()));
        }

        let url = self.build_url("/stats/report")?;
        let response = self.request(
            self.http_client
                .get(&url)
                .query(&[("report_id", report_id)])
        ).await?;

        let stats = response.json().await?;
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn test_get_site_stats() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/stats/site"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "total_subscribers": 1000,
                    "active_subscribers": 950,
                    "growth_rate": 5.5
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.get_site_stats().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_segment_stats() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/stats/segment"))
            .and(query_param("segment_id", "segment_123"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "segment_size": 500,
                    "conversion_rate": 25.5
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.get_segment_stats("segment_123").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_report_stats() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/stats/report"))
            .and(query_param("report_id", "report_123"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "total_views": 1000,
                    "unique_views": 750
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.get_report_stats("report_123").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stats_validation() {
        let mock_server = MockServer::start().await;
        let client = crate::test_utils::create_test_client(mock_server.uri());

        let result = client.get_segment_stats("").await;
        assert!(matches!(result, Err(Error::InvalidSegmentId(_))));

        let result = client.get_report_stats("").await;
        assert!(matches!(result, Err(Error::InvalidRequest(_))));
    }
}