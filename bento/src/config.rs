use crate::error::{Error, Result};
use std::time::Duration;

/// Configuration for the Bento client
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) publishable_key: String,
    pub(crate) secret_key: String,
    pub(crate) site_uuid: String,
    pub(crate) timeout: Duration,
    pub(crate) base_url: String,
}

/// Builder for creating a Config
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    publishable_key: Option<String>,
    secret_key: Option<String>,
    site_uuid: Option<String>,
    timeout: Option<Duration>,
    base_url: Option<String>,
}

impl ConfigBuilder {
    /// Create a new ConfigBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the publishable key
    pub fn publishable_key(mut self, key: impl Into<String>) -> Self {
        self.publishable_key = Some(key.into());
        self
    }

    /// Set the secret key
    pub fn secret_key(mut self, key: impl Into<String>) -> Self {
        self.secret_key = Some(key.into());
        self
    }

    /// Set the site UUID
    pub fn site_uuid(mut self, uuid: impl Into<String>) -> Self {
        self.site_uuid = Some(uuid.into());
        self
    }

    /// Set the timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the base URL
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Build the Config
    pub fn build(self) -> Result<Config> {
        let publishable_key = self.publishable_key
            .ok_or_else(|| Error::InvalidConfig("publishable key is required".into()))?;
        let secret_key = self.secret_key
            .ok_or_else(|| Error::InvalidConfig("secret key is required".into()))?;
        let site_uuid = self.site_uuid
            .ok_or_else(|| Error::InvalidConfig("site UUID is required".into()))?;

        Ok(Config {
            publishable_key,
            secret_key,
            site_uuid,
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            base_url: self.base_url.unwrap_or_else(|| "https://app.bentonow.com/api/v1".into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .publishable_key("pub_key")
            .secret_key("secret_key")
            .site_uuid("site_123")
            .timeout(Duration::from_secs(60))
            .build();

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.publishable_key, "pub_key");
        assert_eq!(config.secret_key, "secret_key");
        assert_eq!(config.site_uuid, "site_123");
        assert_eq!(config.timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_config_builder_missing_required() {
        let config = ConfigBuilder::new().build();
        assert!(config.is_err());

        let config = ConfigBuilder::new()
            .publishable_key("pub_key")
            .build();
        assert!(config.is_err());

        let config = ConfigBuilder::new()
            .publishable_key("pub_key")
            .secret_key("secret_key")
            .build();
        assert!(config.is_err());
    }

    #[test]
    fn test_config_builder_default_values() {
        let config = ConfigBuilder::new()
            .publishable_key("pub_key")
            .secret_key("secret_key")
            .site_uuid("site_123")
            .build()
            .unwrap();

        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.base_url, "https://app.bentonow.com/api/v1");
    }
}