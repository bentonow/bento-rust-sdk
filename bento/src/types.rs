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

/// Data for a subscriber command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandData {
    /// Type of command to execute
    pub command: CommandType,
    /// Email address of the subscriber
    pub email: String,
    /// Query or value for the command
    pub query: String,
}

/// Command execution response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    /// Number of successful operations
    pub results: u32,
    /// Number of failed operations
    pub failed: u32,
}

/// Request body for batch event tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventsRequest {
    /// List of events
    pub events: Vec<EventData>,
}


/// Tracking event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// Event type
    #[serde(rename = "type")]
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

/// Request payload for creating a new subscriber
///
/// Wraps the subscriber data in a container struct as required by the Bento API.
/// Used when making POST requests to create new subscribers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriberRequest {
    /// The subscriber data to be created
    pub subscriber: CreateSubscriberData,
}

// Data required to create a new subscriber
///
/// Contains the minimal required information needed to create a subscriber in the Bento system.
/// Currently only requires an email address, though additional fields can be set after creation
/// using other API endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriberData {
    /// Email address of the subscriber to be created
    ///
    /// Must be a valid email address format. This is the unique identifier
    /// for subscribers in the system.
    pub email: String,
}

/// Import subscriber request data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSubscriberData {
    /// Subscriber email
    pub email: String,
    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Tags to add (comma-separated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Tags to remove (comma-separated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<String>,
    /// Additional custom fields
    #[serde(flatten)]
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Response from a batch subscriber import operation
///
/// Contains information about the success and failure counts from a bulk subscriber import.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSubscriberResponse {
    /// Number of subscribers successfully imported
    pub results: u32,
    /// Number of subscribers that failed to import
    pub failed: u32,
}

/// Subscriber data returned from the API
///
/// Contains the core subscriber information including their unique identifier,
/// data type, and associated attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberData {
    /// Unique identifier for the subscriber
    pub id: String,
    /// Type of the data object, typically "subscriber"
    #[serde(rename = "type")]
    pub data_type: String,
    /// Detailed attributes associated with the subscriber
    pub attributes: SubscriberAttributes,
}

/// Detailed attributes for a subscriber
///
/// Contains all the mutable and configurable properties of a subscriber,
/// including their contact information, custom fields, tags, and subscription status.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriberAttributes {
    /// Unique UUID for the subscriber
    pub uuid: String,
    /// Email address of the subscriber
    pub email: String,
    /// Custom fields associated with the subscriber
    ///
    /// The fields are stored as key-value pairs where the value can be any valid JSON value
    pub fields: HashMap<String, serde_json::Value>,
    /// List of tag IDs currently applied to the subscriber
    pub cached_tag_ids: Vec<String>,
    /// Timestamp when the subscriber unsubscribed, if applicable
    ///
    /// None if the subscriber is currently subscribed
    #[serde(with = "time::serde::rfc3339::option")]
    pub unsubscribed_at: Option<OffsetDateTime>,
}

/// API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Response data
    pub data: T,
}