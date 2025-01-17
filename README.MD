# Bento Rust SDK
<img align="right" src="https://app.bentonow.com/brand/logoanim.gif">

> [!TIP]
> Need help? Join our [Discord](https://discord.gg/ssXXFRmt5F) or email jesse@bentonow.com for personalized support.

The Bento Rust SDK makes it quick and easy to send emails and track events in your Rust applications. We provide powerful and customizable APIs that can be used out-of-the-box to manage subscribers, track events, and send transactional emails.

Get started with our [📚 integration guides](https://docs.bentonow.com), or [📘 browse the SDK reference](https://docs.bentonow.com/subscribers).

## Features

* **Event Tracking**: Easily track custom events and user behavior in your Rust applications
* **Subscriber Management**: Import and manage subscribers directly using type-safe structures
* **Idiomatic Rust**: Designed using Rust best practices and patterns
* **Async Support**: All operations are async and support cancellation via futures
* **Strong Types**: Type-safe request and response handling with serde
* **Retry Logic**: Built-in retry handling for rate limits and transient failures

## Requirements

- Rust 1.70 or higher
- Bento API Keys

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bento = "0.1.0"
```

## Quick Start

```rust
use bento::{Client, Config, ConfigBuilder};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::new()
        .publishable_key("your-key")
        .secret_key("your-secret")
        .site_uuid("your-uuid")
        .build()?;

    let client = Client::new(config)?;

    let subscriber = client.find_subscriber("test@example.com").await?;
    println!("Found subscriber: {:?}", subscriber);

    Ok(())
}
```

## Core APIs

### Subscriber Management

#### Find Subscriber
Retrieves a subscriber by their email address:

```rust
let subscriber = client.find_subscriber("test@example.com").await?;
println!("Subscriber details: {:?}", subscriber);
```

#### Create Subscriber
Creates a new subscriber in your account:

```rust
use std::collections::HashMap;

let mut fields = HashMap::new();
fields.insert("company".to_string(), serde_json::json!("Acme Inc"));
fields.insert("role".to_string(), serde_json::json!("Developer"));

let input = SubscriberInput {
    email: "test@example.com".to_string(),
    first_name: Some("John".to_string()),
    last_name: Some("Doe".to_string()),
    tags: Some(vec!["new-user".to_string()]),
    fields: Some(fields),
    remove_tags: None,
};

let new_subscriber = client.create_subscriber(input).await?;
println!("Created subscriber: {:?}", new_subscriber);
```

#### Import Subscribers
Batch import multiple subscribers:

```rust
let subscribers = vec![
    SubscriberInput {
        email: "user1@example.com".to_string(),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        tags: Some(vec!["imported".to_string(), "customer".to_string()]),
        fields: Some({
            let mut fields = HashMap::new();
            fields.insert("imported_at".to_string(), 
                serde_json::json!(chrono::Utc::now().to_rfc3339()));
            fields
        }),
        remove_tags: None,
    },
    SubscriberInput {
        email: "user2@example.com".to_string(),
        first_name: Some("Jane".to_string()),
        last_name: Some("Smith".to_string()),
        tags: Some(vec!["imported".to_string(), "prospect".to_string()]),
        fields: Some({
            let mut fields = HashMap::new();
            fields.insert("imported_at".to_string(), 
                serde_json::json!(chrono::Utc::now().to_rfc3339()));
            fields
        }),
        remove_tags: None,
    },
];

client.import_subscribers(subscribers).await?;
```

### Event Tracking

#### Track Events
Send custom events to track user behavior:

```rust
let mut fields = HashMap::new();
fields.insert("onboarding_type".to_string(), serde_json::json!("api_test"));
fields.insert("timestamp".to_string(), 
    serde_json::json!(chrono::Utc::now().to_rfc3339()));

let mut details = HashMap::new();
details.insert("source".to_string(), serde_json::json!("api"));
details.insert("version".to_string(), serde_json::json!("1.0"));

let event = EventData {
    event_type: "$completed_onboarding".to_string(),
    email: "user@example.com".to_string(),
    fields: Some(fields),
    details: Some(details),
};

client.track_events(vec![event]).await?;
```

### Email Management

#### Send Transactional Emails
Send personalized transactional emails:

```rust
let mut personalizations = HashMap::new();
personalizations.insert("name".to_string(), serde_json::json!("John Doe"));
personalizations.insert("account_type".to_string(), serde_json::json!("premium"));

let email = EmailData {
    to: "recipient@example.com".to_string(),
    from: "sender@yourdomain.com".to_string(),
    subject: "Welcome to Our Service".to_string(),
    html_body: "<p>Hello {{ name }}, welcome aboard!</p>".to_string(),
    transactional: true,
    personalizations: Some(personalizations),
};

let batch = EmailBatch::new(vec![email])?;
let results = client.send_emails(batch).await?;
println!("Successfully queued {} emails for delivery", results);
```

#### Send Multiple Emails
Send multiple emails in a single request:

```rust
let emails = vec![
    EmailData {
        to: "user1@example.com".to_string(),
        from: "notifications@yourdomain.com".to_string(),
        subject: "Your Daily Update".to_string(),
        html_body: "<p>Hi {{ name }}, here's your daily summary...</p>".to_string(),
        transactional: true,
        personalizations: Some({
            let mut p = HashMap::new();
            p.insert("name".to_string(), serde_json::json!("User 1"));
            p.insert("summary_items".to_string(), 
                serde_json::json!(vec!["item1", "item2"]));
            p
        }),
    },
    EmailData {
        to: "user2@example.com".to_string(),
        from: "notifications@yourdomain.com".to_string(),
        subject: "Your Daily Update".to_string(),
        html_body: "<p>Hi {{ name }}, here's your daily summary...</p>".to_string(),
        transactional: true,
        personalizations: Some({
            let mut p = HashMap::new();
            p.insert("name".to_string(), serde_json::json!("User 2"));
            p.insert("summary_items".to_string(), 
                serde_json::json!(vec!["item3", "item4"]));
            p
        }),
    },
];

let batch = EmailBatch::new(emails)?;
let results = client.send_emails(batch).await?;
```

### Broadcast Management

#### Get Broadcasts
Retrieve a list of all broadcasts in your account:

```rust
let broadcasts = client.get_broadcasts().await?;
for broadcast in broadcasts {
    println!(
        "Broadcast: {} (Type: {:?})", 
        broadcast.name, 
        broadcast.broadcast_type
    );
}
```

#### Create Broadcasts
Create new broadcast campaigns:

```rust
let broadcast = BroadcastData {
    name: "Campaign #1 Example".to_string(),
    subject: "Hello world Plain Text".to_string(),
    content: "<p>Hi {{ visitor.first_name }}</p>".to_string(),
    broadcast_type: BroadcastType::Plain,
    from: ContactData {
        name: Some("John Doe".to_string()),
        email: "sender@yourdomain.com".to_string(),
    },
    inclusive_tags: Some("lead,mql".to_string()),
    exclusive_tags: Some("customers".to_string()),
    segment_id: Some("segment_123456789".to_string()),
    batch_size_per_hour: 1500,
};

client.create_broadcasts(vec![broadcast]).await?;
```

### Tag Management

#### Get Tags
Retrieve all tags in your account:

```rust
let tags = client.get_tags().await?;
for tag in tags {
    println!(
        "Tag: {} (ID: {}, Created: {})",
        tag.attributes.name,
        tag.id,
        tag.attributes.created_at
    );
}
```

#### Create Tag
Create a new tag:

```rust
let new_tag = client.create_tag("rust-sdk-test-tag").await?;
println!("Created new tag: {}", new_tag.attributes.name);
```

### Field Management

#### Get Fields
Retrieve all custom fields:

```rust
let fields = client.get_fields().await?;
for field in fields {
    println!("Field: {}", field.attributes.key);
    println!("  Name: {}", field.attributes.name);
    if let Some(created_at) = field.attributes.created_at {
        println!("  Created: {}", created_at);
    }
}
```

#### Create Field
Create a new custom field:

```rust
let new_field = client.create_field("purchase_amount").await?;
println!("Created new field: {}", new_field.attributes.key);
```

### Statistics APIs

#### Get Site Stats
Retrieve overall statistics for your site:

```rust
let stats = client.get_site_stats().await?;
println!("Site statistics: {:?}", stats);
```

#### Get Segment Stats
Retrieve statistics for a specific segment:

```rust
let segment_stats = client.get_segment_stats("segment_123").await?;
println!("Segment stats: {:?}", segment_stats);
```

#### Get Report Stats
Retrieve statistics for a specific report:

```rust
let report_stats = client.get_report_stats("report_456").await?;
println!("Report stats: {:?}", report_stats);
```

### Experimental APIs

#### Blacklist Status Check
Check if an IP address or domain is blacklisted:

```rust
let blacklist_data = BlacklistData {
    domain: Some("example.com".to_string()),
    ip: Some("1.1.1.1".to_string()),
};

let result = client.get_blacklist_status(&blacklist_data).await?;
println!("Blacklist check result: {:?}", result);
```

#### Email Validation
Validate email addresses with additional context:

```rust
let validation_data = ValidationData {
    email: "test@example.com".to_string(),
    name: Some("John Snow".to_string()),
    user_agent: Some("Rust-SDK-Test".to_string()),
    ip: Some("1.1.1.1".to_string()),
};

let result = client.validate_email(&validation_data).await?;
println!("Email validation result: valid={}", result.valid);
```

### Error Handling

The SDK provides strongly typed errors for better error handling:

```rust
use bento::Error;

match client.create_tag("test-tag").await {
    Ok(tag) => println!("Created tag: {}", tag.attributes.name),
    Err(Error::InvalidConfig(msg)) => eprintln!("Configuration error: {}", msg),
    Err(Error::InvalidEmail(email)) => eprintln!("Invalid email: {}", email),
    Err(Error::InvalidIpAddress(ip)) => eprintln!("Invalid IP address: {}", ip),
    Err(Error::UnexpectedResponse(msg)) => eprintln!("API error: {}", msg),
    Err(Error::RateLimit) => eprintln!("Rate limit exceeded"),
    Err(e) => eprintln!("Other error: {}", e),
}
```

Available error types:
- `Error::InvalidConfig`: Configuration error
- `Error::InvalidEmail`: Invalid email format
- `Error::InvalidIpAddress`: Invalid IP address format
- `Error::InvalidRequest`: Invalid request parameters
- `Error::UnexpectedResponse`: Unexpected API response
- `Error::InvalidName`: Invalid name format
- `Error::InvalidSegmentId`: Invalid segment ID
- `Error::InvalidContent`: Invalid content
- `Error::InvalidTags`: Invalid tags format
- `Error::InvalidBatchSize`: Invalid batch size
- `Error::RateLimit`: Rate limit exceeded
- `Error::AuthenticationFailed`: Authentication failed
- `Error::HttpClient`: HTTP client error

## Best Practices

### Async Runtime Usage
The SDK requires a Tokio runtime:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::new()
        .publishable_key("your-key")
        .secret_key("your-secret")
        .site_uuid("your-uuid")
        .build()?;

    let client = Client::new(config)?;
    
    // Your async code here
    
    Ok(())
}
```

### Error Handling
Always match on specific error types for better error handling:

```rust
match client.create_subscriber(subscriber_input).await {
    Ok(subscriber) => {
        // Handle successful creation
    }
    Err(Error::InvalidEmail(email)) => {
        // Handle invalid email specifically
    }
    Err(Error::RateLimit) => {
        // Handle rate limiting
        tokio::time::sleep(Duration::from_secs(1)).await;
        // Retry the request
    }
    Err(e) => {
        // Handle other errors
        eprintln!("Error: {}", e);
    }
}
```

### Batch Operations
When performing batch operations, respect the limits:
- Maximum 60 emails per batch
- Use reasonable batch sizes for subscriber imports

```rust
// Split large imports into chunks
for chunk in subscribers.chunks(500) {
    match client.import_subscribers(chunk.to_vec()).await {
        Ok(_) => println!("Imported chunk successfully"),
        Err(e) => eprintln!("Failed to import chunk: {}", e),
    }
}
```

## Things to Know

1. All API methods are async and require a Tokio runtime
2. All methods perform input validation before making API calls
3. Strong types ensure type safety for requests and responses
4. Error handling follows Rust best practices
5. The SDK uses reqwest for HTTP requests with configurable timeouts
6. All responses are properly deserialized using serde
7. The SDK is thread-safe and supports concurrent usage
8. Built-in retry logic for rate limits and transient failures
9. All methods have comprehensive tests

## Contributing

We welcome contributions! Please feel free to submit a Pull Request. Here are some ways you can help:

- Report bugs and issues
- Add new features
- Improve documentation
- Add tests
- Format code with rustfmt
- Run clippy for linting improvements
- Add API documentation
- Provide feedback on API design
- Share example code

## Development

To build and test the SDK locally:

```bash
# Build the SDK
cargo build

# Run tests
cargo test

# Run with all features enabled
cargo test --all-features

# Check formatting
cargo fmt -- --check

# Run clippy lints
cargo clippy -- -D warnings
```

## Running Tests

The test suite includes both unit tests and integration tests:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

## License

The Bento SDK for Rust is available as open source under the terms of the [MIT License](LICENSE.md).

## Support

Need help? Here are some ways to get support:

- Join our [Discord](https://discord.gg/ssXXFRmt5F)
- Email support at jesse@bentonow.com
- Check out our [documentation](https://docs.bentonow.com)
- Open an issue on GitHub