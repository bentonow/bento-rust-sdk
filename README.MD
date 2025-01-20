# Bento Rust SDK
<img align="right" src="https://app.bentonow.com/brand/logoanim.gif">

> [!TIP]
> Need help? Join our [Discord](https://discord.gg/ssXXFRmt5F) or email jesse@bentonow.com for personalized support.

The Bento Rust SDK makes it quick and easy to send emails and track events in your Rust applications. We provide powerful and customizable APIs that can be used out-of-the-box to manage subscribers, track events, and send transactional emails.

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

Add this to your `Cargo.toml`:

```toml
[dependencies]
bento = "0.1.0"
```

## Quick Start

```rust
use bento::{Client, ConfigBuilder};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::new()
        .publishable_key("your-key")
        .secret_key("your-secret")
        .site_uuid("your-uuid")
        .build()?;

    let client = Client::new(config)?;

    // Create a simple subscriber
    let new_subscriber = client.create_subscriber("test@example.com").await?;
    println!("Created subscriber: {:?}", new_subscriber);

    Ok(())
}
```

## Core APIs

### Subscriber Management

The SDK provides three main methods for subscriber management:

#### Find Subscriber
```rust
let subscriber = client.find_subscriber("test@example.com").await?;
println!("Found subscriber: {:?}", subscriber);
```

#### Create Subscriber
```rust
// Simple subscriber creation
let subscriber = client.create_subscriber("test@example.com").await?;
```

#### Import Subscribers
```rust
use std::collections::HashMap;
use bento::ImportSubscriberData;

// Create subscriber with full data
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

### Email Management

The SDK supports sending emails in batches using the `EmailBatch` struct:

```rust
use bento::{EmailData, EmailBatch};

let email = EmailData {
    to: "recipient@example.com".to_string(),
    from: "sender@yourdomain.com".to_string(),
    subject: "Welcome".to_string(),
    html_body: "<p>Hello!</p>".to_string(),
    transactional: true,
    personalizations: None,
};

// Create a batch (maximum 60 emails per batch)
let mut batch = EmailBatch::new(vec![email])?;

// Add another email to the batch
let another_email = EmailData {
    to: "another@example.com".to_string(),
    from: "sender@yourdomain.com".to_string(),
    subject: "Welcome".to_string(),
    html_body: "<p>Hello!</p>".to_string(),
    transactional: true,
    personalizations: None,
};

batch.add_email(another_email)?;
```

### Event Tracking

Track custom events with rich metadata:

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

Create and manage email broadcasts:

```rust
use bento::{BroadcastData, BroadcastType, ContactData};

let broadcast = BroadcastData {
    name: "Test Campaign".to_string(),
    subject: "Welcome Email".to_string(),
    content: "<p>Hello subscribers!</p>".to_string(),
    broadcast_type: BroadcastType::Plain,
    from: ContactData {
        name: Some("John Doe".to_string()),
        email: "sender@yourdomain.com".to_string(),
    },
    inclusive_tags: None,
    exclusive_tags: None,
    segment_id: None,
    batch_size_per_hour: 1000,
};

client.create_broadcasts(vec![broadcast]).await?;
```

## Error Handling

The SDK provides strongly typed errors:

```rust
use bento::Error;

match client.create_subscriber("test@example.com").await {
    Ok(subscriber) => println!("Created: {}", subscriber.id),
    Err(Error::InvalidEmail(email)) => eprintln!("Invalid email: {}", email),
    Err(Error::RateLimit) => eprintln!("Rate limit exceeded"),
    Err(Error::AuthenticationFailed) => eprintln!("Auth failed"),
    Err(e) => eprintln!("Other error: {}", e),
}
```

Available error types:
- `Error::InvalidConfig`: Configuration error
- `Error::InvalidEmail`: Invalid email format
- `Error::InvalidRequest`: Invalid request parameters
- `Error::UnexpectedResponse`: Unexpected API response
- `Error::InvalidBatchSize`: Invalid batch size
- `Error::RateLimit`: Rate limit exceeded
- `Error::AuthenticationFailed`: Authentication failed
- `Error::HttpClient`: HTTP client error

## Best Practices

### Configuration
Use the builder pattern for client configuration:

```rust
let config = ConfigBuilder::new()
    .publishable_key("your-key")
    .secret_key("your-secret")
    .site_uuid("your-uuid")
    .timeout(Duration::from_secs(30)) // Optional
    .base_url("https://custom.url") // Optional
    .build()?;
```

### Batch Operations
Respect the email batch size limit:
```rust
// EmailBatch has a hard limit of 60 emails
if emails.len() > 60 {
    for chunk in emails.chunks(60) {
        let batch = EmailBatch::new(chunk.to_vec())?;
        // Process batch
    }
}
```

### Error Handling
Always handle rate limits and authentication errors:
```rust
match client.find_subscriber("test@example.com").await {
    Ok(subscriber) => {
        // Process subscriber
    }
    Err(Error::RateLimit) => {
        tokio::time::sleep(Duration::from_secs(1)).await;
        // Retry the request
    }
    Err(Error::AuthenticationFailed) => {
        // Check your API keys
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
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