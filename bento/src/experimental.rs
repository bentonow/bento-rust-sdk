use crate::{Client, Error, Result};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tracing::instrument;

/// Data for blacklist status checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistData {
    /// Domain to check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// IP address to check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// Data for email validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationData {
    /// Email address to validate
    pub email: String,
    /// Full name of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User agent string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// IP address of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// Response from email validation
#[derive(Debug, Clone, Deserialize)]
pub struct ValidationResponse {
    /// Whether the email is valid
    pub valid: bool,
}

impl Client {
    /// Check domain or IP address blacklist status
    #[instrument(skip(self))]
    pub async fn get_blacklist_status(&self, data: &BlacklistData) -> Result<serde_json::Value> {
        if data.domain.is_none() && data.ip.is_none() {
            return Err(Error::InvalidRequest("Either domain or IP is required".into()));
        }

        if let Some(ip) = &data.ip {
            if ip.parse::<IpAddr>().is_err() {
                return Err(Error::InvalidIpAddress(ip.clone()));
            }
        }

        let url = self.build_url("/experimental/blacklist.json")?;
        let mut request = self.http_client.get(&url);

        if let Some(domain) = &data.domain {
            request = request.query(&[("domain", domain)]);
        }
        if let Some(ip) = &data.ip {
            request = request.query(&[("ip", ip)]);
        }

        let response = self.request(request).await?;
        let result = response.json().await?;
        Ok(result)
    }

    /// Validate email address with additional context
    #[instrument(skip(self))]
    pub async fn validate_email(&self, data: &ValidationData) -> Result<ValidationResponse> {
        if !data.email.contains('@') {
            return Err(Error::InvalidEmail(data.email.clone()));
        }

        if let Some(ip) = &data.ip {
            if ip.parse::<IpAddr>().is_err() {
                return Err(Error::InvalidIpAddress(ip.clone()));
            }
        }

        let url = self.build_url("/experimental/validation")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .json(data)
        ).await?;

        let result = response.json().await?;
        Ok(result)
    }

    /// Moderate content
    #[instrument(skip(self))]
    pub async fn get_content_moderation(&self, content: &str) -> Result<serde_json::Value> {
        if content.is_empty() {
            return Err(Error::InvalidContent("Content is required".into()));
        }

        let url = self.build_url("/experimental/content_moderation")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .query(&[("content", content)])
        ).await?;

        let result = response.json().await?;
        Ok(result)
    }

    /// Predict gender from name
    #[instrument(skip(self))]
    pub async fn get_gender(&self, name: &str) -> Result<serde_json::Value> {
        if name.is_empty() {
            return Err(Error::InvalidName("Name is required".into()));
        }

        let url = self.build_url("/experimental/gender")?;
        let response = self.request(
            self.http_client
                .post(&url)
                .query(&[("name", name)])
        ).await?;

        let result = response.json().await?;
        Ok(result)
    }

    /// Geolocate IP address
    #[instrument(skip(self))]
    pub async fn geolocate_ip(&self, ip: &str) -> Result<serde_json::Value> {
        if ip.parse::<IpAddr>().is_err() {
            return Err(Error::InvalidIpAddress(ip.to_string()));
        }

        let url = self.build_url("/experimental/geolocation")?;
        let response = self.request(
            self.http_client
                .get(&url)
                .query(&[("ip", ip)])
        ).await?;

        let result = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn test_blacklist_check() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/experimental/blacklist.json"))
            .and(query_param("domain", "example.com"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "status": "clean"
                })))
            .mount(&mock_server)
            .await;

        let client = crate::test_utils::create_test_client(mock_server.uri());
        let result = client.get_blacklist_status(&BlacklistData {
            domain: Some("example.com".into()),
            ip: None,
        }).await;

        assert!(result.is_ok());
    }

    // Additional tests...
}