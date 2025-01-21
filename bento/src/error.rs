use thiserror::Error;

/// Errors that can occur when using the Bento API
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid configuration provided
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    /// Invalid email address format
    #[error("invalid email address: {0}")]
    InvalidEmail(String),

    /// Invalid IP address format
    #[error("invalid IP address: {0}")]
    InvalidIpAddress(String),

    /// Invalid request parameters
    #[error("invalid request parameters: {0}")]
    InvalidRequest(String),

    /// Unexpected API response
    #[error("unexpected API response: {0}")]
    UnexpectedResponse(String),

    /// Invalid command type
    #[error("invalid command type: {0}")]
    InvalidCommand(String),

    /// Invalid name format
    #[error("invalid name format: {0}")]
    InvalidName(String),

    /// Invalid segment ID
    #[error("invalid segment ID: {0}")]
    InvalidSegmentId(String),

    /// Invalid content
    #[error("invalid content: {0}")]
    InvalidContent(String),

    /// Invalid tags format
    #[error("invalid tags format: {0}")]
    InvalidTags(String),

    /// Invalid batch size
    #[error("invalid batch size: {0}")]
    InvalidBatchSize(String),

    /// HTTP client error
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    /// Rate limit exceeded
    #[error("rate limit exceeded")]
    RateLimit,

    /// Authentication failed
    #[error("authentication failed")]
    AuthenticationFailed,
}

/// Result type for Bento operations
pub type Result<T> = std::result::Result<T, Error>;