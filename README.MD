# Bento Rust SDK
<img align="right" src="https://app.bentonow.com/brand/logoanim.gif">

> [!TIP]
> Need help? Join our [Discord](https://discord.gg/ssXXFRmt5F) or email jesse@bentonow.com for personalized support.

The Bento Rust SDK provides a robust interface for email management, subscriber tracking, event handling and more for your Rust applications. We provide powerful and customizable APIs that can be used out-of-the-box.

Get started with our [📚 integration guides](https://docs.bentonow.com), or [📘 browse the SDK reference](https://docs.bentonow.com/subscribers).

## Features

* **Event Tracking**: Track custom events with rich metadata and fields
* **Subscriber Management**: Create and import subscribers with full data support
* **Email Handling**: Send transactional emails with batching support
* **Broadcast Management**: Create and manage email broadcasts
* **Async Support**: Built on tokio for async/await support
* **Strong Types**: Type-safe request and response handling with serde
* **Retry Logic**: Automatic retry handling for rate limits

## Requirements

- Rust 1.70 or higher
- Bento API Keys (Publishable Key and Secret Key)
- Site UUID

## Installation

1. Add to your `Cargo.toml`:
```toml
[dependencies]
bento = "0.1.0"
```

2. Create a `.env` file in your project root:
```env
BENTO_PUBLISHABLE_KEY=your_publishable_key
BENTO_SECRET_KEY=your_secret_key
BENTO_SITE_UUID=your_site_uuid
```

## Quick Start

```rust
use bento::{Client, ConfigBuilder};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Initialize client
    let config = ConfigBuilder::new()
        .publishable_key(&env::var("BENTO_PUBLISHABLE_KEY")?)
        .secret_key(&env::var("BENTO_SECRET_KEY")?)
        .site_uuid(&env::var("BENTO_SITE_UUID")?)
        .build()?;

    let client = Client::new(config)?;

    Ok(())
}
```

## Core APIs

### Subscriber Management

```rust
use std::collections::HashMap;
use bento::ImportSubscriberData;

// Find a subscriber
let subscriber = client.find_subscriber("test@example.com").await?;

// Create a simple subscriber
let new_subscriber = client.create_subscriber("test@example.com").await?;

// Import subscribers with full data
let mut custom_fields = HashMap::new();
custom_fields.insert("company".to_string(), serde_json::json!("Acme Inc"));
custom_fields.insert("role".to_string(), serde_json::json!("Developer"));

let subscriber = ImportSubscriberData {
    email: "test@example.com".to_string(),
    first_name: Some("John".to_string()),
    last_name: Some("Doe".to_string()),
    tags: Some("lead,mql".to_string()),
    remove_tags: Some("customer".to_string()),
    custom_fields,
};

client.import_subscribers(vec![subscriber]).await?;
```

### Subscriber Commands

#### Available Command Types
- `AddTag` - Add a tag to a subscriber
- `AddTagViaEvent` - Add a tag via an event
- `RemoveTag` - Remove a tag from a subscriber
- `AddField` - Add a field to a subscriber
- `RemoveField` - Remove a field from a subscriber
- `Subscribe` - Subscribe a user
- `Unsubscribe` - Unsubscribe a user
- `ChangeEmail` - Change a user's email

#### Execute Single Command
```rust
use bento::{Client, CommandData, CommandType};

let command = CommandData {
    command: CommandType::AddTag,
    email: "user@example.com".to_string(),
    query: "new-customer".to_string(),
};

client.subscriber_command(vec![command]).await?;
```

#### Execute Multiple Commands
```rust
let commands = vec![
    CommandData {
        command: CommandType::AddTag,
        email: "user@example.com".to_string(),
        query: "active".to_string(),
    },
    CommandData {
        command: CommandType::AddField,
        email: "user@example.com".to_string(),
        query: "signup_date=2024-01-20".to_string(),
    },
];

client.subscriber_command(commands).await?;
```

### Email Management

```rust
use bento::{EmailData, EmailBatch};

// Create an email
let email = EmailData {
    to: "recipient@example.com".to_string(),
    from: "sender@yourdomain.com".to_string(),
    subject: "Welcome".to_string(),
    html_body: "<p>Hello!</p>".to_string(),
    transactional: true,
    personalizations: Some({
        let mut map = HashMap::new();
        map.insert("name".to_string(), serde_json::json!("John"));
        map
    }),
};

// Create a batch (maximum 60 emails per batch)
let batch = EmailBatch::new(vec![email])?;
```

### Event Tracking

```rust
use bento::EventData;

let event = EventData {
    event_type: "$completed_onboarding".to_string(),
    email: "user@example.com".to_string(),
    fields: Some({
        let mut fields = HashMap::new();
        fields.insert("source".to_string(), serde_json::json!("api"));
        fields
    }),
    details: Some({
        let mut details = HashMap::new();
        details.insert("version".to_string(), serde_json::json!("1.0"));
        details
    }),
};

client.track_events(vec![event]).await?;
```

### Broadcast Management

```rust
use bento::{BroadcastData, BroadcastType, ContactData};

// Get all broadcasts
let broadcasts = client.get_broadcasts().await?;

// Create a broadcast
let broadcast = BroadcastData {
    name: "Test Campaign".to_string(),
    subject: "Welcome Email".to_string(),
    content: "<p>Hello subscribers!</p>".to_string(),
    broadcast_type: BroadcastType::Plain,
    from: ContactData {
        name: Some("John Doe".to_string()),
        email: "sender@yourdomain.com".to_string(),
    },
    inclusive_tags: Some("lead,mql".to_string()),
    exclusive_tags: None,
    segment_id: None,
    batch_size_per_hour: 300, // Default batch size
};

client.create_broadcasts(vec![broadcast]).await?;
```

### Tag Management

```rust
// Get all tags
let tags = client.get_tags().await?;

// Create a new tag
let new_tag = client.create_tag("new-customer").await?;
```

### Field Management

```rust
// Get all custom fields
let fields = client.get_fields().await?;

// Create a new custom field
let new_field = client.create_field("purchase_amount").await?;
```

### Statistics APIs

```rust
// Get site-wide statistics
let site_stats = client.get_site_stats().await?;

// Get segment statistics
let segment_stats = client.get_segment_stats("segment_123").await?;

// Get report statistics
let report_stats = client.get_report_stats("report_456").await?;
```

### Experimental APIs

```rust
use bento::{BlacklistData, ValidationData};

// Check domain blacklist status
let domain_check = client.get_blacklist_status(&BlacklistData {
    domain: Some("example.com".to_string()),
    ip: None,
}).await?;

// Check IP blacklist status
let ip_check = client.get_blacklist_status(&BlacklistData {
    domain: None,
    ip: Some("1.1.1.1".to_string()),
}).await?;

// Validate email
let validation = client.validate_email(&ValidationData {
    email: "test@example.com".to_string(),
    name: Some("John Doe".to_string()),
    user_agent: Some("Mozilla/5.0".to_string()),
    ip: Some("1.1.1.1".to_string()),
}).await?;

// Content moderation
let moderation = client.get_content_moderation("Content to moderate").await?;

// Gender prediction
let gender = client.get_gender("John").await?;

// IP geolocation
let location = client.geolocate_ip("1.1.1.1").await?;
```

## Data Types

### Broadcast Types
```rust
pub enum BroadcastType {
    Plain,  // Plain text broadcast
    Raw,    // Raw HTML broadcast
}
```

### Command Types
```rust
pub enum CommandType {
    AddTag,          // Add a tag to a subscriber
    AddTagViaEvent,  // Add a tag via an event
    RemoveTag,       // Remove a tag from a subscriber
    AddField,        // Add a field to a subscriber
    RemoveField,     // Remove a field from a subscriber
    Subscribe,       // Subscribe a user
    Unsubscribe,     // Unsubscribe a user
    ChangeEmail,     // Change a user's email
}
```

### Subscriber Data
```rust
pub struct SubscriberData {
    pub id: String,
    pub data_type: String,
    pub attributes: SubscriberAttributes,
}

pub struct SubscriberAttributes {
    pub uuid: String,
    pub email: String,
    pub fields: HashMap<String, serde_json::Value>,
    pub cached_tag_ids: Vec<String>,
    pub unsubscribed_at: Option<OffsetDateTime>,
}
```

### Email Data
```rust
pub struct EmailData {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub html_body: String,
    pub transactional: bool,
    pub personalizations: Option<HashMap<String, serde_json::Value>>,
}
```

## Error Handling

The SDK includes built-in retry logic with exponential backoff for rate limits and transient failures. Available error types:

```rust
pub enum Error {
    InvalidConfig(String),        // Configuration error
    InvalidEmail(String),         // Invalid email format
    InvalidIpAddress(String),     // Invalid IP address
    InvalidRequest(String),       // Invalid request parameters
    UnexpectedResponse(String),   // Unexpected API response
    InvalidName(String),          // Invalid name format
    InvalidSegmentId(String),     // Invalid segment ID
    InvalidContent(String),       // Invalid content
    InvalidTags(String),         // Invalid tags format
    InvalidBatchSize(String),    // Invalid batch size
    HttpClient(reqwest::Error),  // HTTP client error
    RateLimit,                   // Rate limit exceeded
    AuthenticationFailed,        // Authentication failed
}
```

## Contributing

We welcome contributions! Please feel free to submit a Pull Request.

## License

This SDK is available under the MIT License.

## Support

Need help? Here are some ways to get support:

- Join our [Discord](https://discord.gg/ssXXFRmt5F)
- Email support at jesse@bentonow.com
- Check out our [documentation](https://docs.bentonow.com)
- Open an issue on GitHub