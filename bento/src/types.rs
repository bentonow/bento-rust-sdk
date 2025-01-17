use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

/// Type for broadcast messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BroadcastType {
    /// Plain text broadcast
    Plain,
    /// Raw HTML broadcast
    Raw,
}

/// Type for subscriber commands
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandType {
    /// Add a tag to a subscriber
    AddTag,
    /// Add a tag via an event
    AddTagViaEvent,
    /// Remove a tag from a subscriber
    RemoveTag,
    /// Add a field to a subscriber
    AddField,
    /// Remove a field from a subscriber
    RemoveField,
    /// Subscribe a user
    Subscribe,
    /// Unsubscribe a user
    Unsubscribe,
    /// Change a user's email
    ChangeEmail,
}

/// Tracking event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// Event type
    pub event_type: String,
    /// Subscriber email
    pub email: String,
    /// Additional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, serde_json::Value>>,
    /// Event details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactData {
    /// Contact name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Contact email
    pub email: String,
}

/// Broadcast message data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastData {
    /// Broadcast name
    pub name: String,
    /// Email subject
    pub subject: String,
    /// Message content
    pub content: String,
    /// Broadcast type
    #[serde(rename = "type")]
    pub broadcast_type: BroadcastType,
    /// Sender information
    pub from: ContactData,
    /// Tags to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive_tags: Option<String>,
    /// Tags to exclude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_tags: Option<String>,
    /// Segment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    /// Batch size per hour
    pub batch_size_per_hour: u32,
}

/// Single email message data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailData {
    /// Recipient email
    pub to: String,
    /// Sender email
    pub from: String,
    /// Email subject
    pub subject: String,
    /// HTML content
    pub html_body: String,
    /// Whether this is a transactional email
    pub transactional: bool,
    /// Personalization data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalizations: Option<HashMap<String, serde_json::Value>>,
}

/// Subscriber data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberData {
    /// Subscriber ID
    pub id: String,
    /// Data type
    #[serde(rename = "type")]
    pub data_type: String,
    /// Subscriber attributes
    pub attributes: SubscriberAttributes,
}

/// Subscriber attributes
#[derive(Debug, Clone, Serialize, Deserialize, Default)] // Add Default here
pub struct SubscriberAttributes {
    /// UUID
    pub uuid: String,
    /// Email address
    pub email: String,
    /// Custom fields
    pub fields: HashMap<String, serde_json::Value>,
    /// Assigned tag IDs
    pub cached_tag_ids: Vec<String>,
    /// Unsubscribe date
    #[serde(with = "time::serde::rfc3339::option")]
    pub unsubscribed_at: Option<OffsetDateTime>,
}

/// API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Response data
    pub data: T,
}