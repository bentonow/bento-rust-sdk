use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

use bento::{Client, ConfigBuilder, EventData, BroadcastData, BroadcastType, ContactData, ImportSubscriberData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// Load environment variables
    dotenv().ok();

    /// Initialize client
    let config = ConfigBuilder::new()
        .publishable_key(&env::var("BENTO_PUBLISHABLE_KEY")?)
        .secret_key(&env::var("BENTO_SECRET_KEY")?)
        .site_uuid(&env::var("BENTO_SITE_UUID")?)
        .build()?;

    let client = Client::new(config)?;

    /// Find a subscriber by email
    let subscriber = client.find_subscriber("rust@example.com").await?;
    println!("Found subscriber: {:?}", subscriber);

    /// Create subscriber example (simple version)
    let new_subscriber = client.create_subscriber("rust@example.com").await?;
    println!("Created subscriber: {:?}", new_subscriber);

    /// Import subscribers example with full data
    let mut custom_fields = HashMap::new();
    custom_fields.insert("company".to_string(), serde_json::json!("Acme Inc"));
    custom_fields.insert("role".to_string(), serde_json::json!("Developer"));

    let subscriber = ImportSubscriberData {
        email: "rust@example.com".to_string(),
        first_name: Some("Rust".to_string()),
        last_name: Some("sdk".to_string()),
        tags: Some("lead,mql".to_string()),
        remove_tags: Some("customer".to_string()),
        custom_fields,
    };

    client.import_subscribers(vec![subscriber]).await?;

    /// Track event example
    let event = EventData {
        event_type: "$onboarding_complete".to_string(),
        email: "test@example.com".to_string(),
        fields: Some(HashMap::new()),
        details: Some(HashMap::new()),
    };

    match client.track_events(vec![event]).await {
        Ok(_) => println!("Successfully tracked event"),
        Err(e) => eprintln!("Error tracking event: {}", e),
    }

    /// Create broadcast example
    let broadcast = BroadcastData {
        name: "Test Campaign".to_string(),
        subject: "Test Broadcast".to_string(),
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

    match client.create_broadcasts(vec![broadcast]).await {
        Ok(_) => println!("Successfully created broadcast"),
        Err(e) => eprintln!("Error creating broadcast: {}", e),
    }

    /// Get tags example
    match client.get_tags().await {
        Ok(tags) => println!("Tags: {:?}", tags),
        Err(e) => eprintln!("Error getting tags: {}", e),
    }

    /// Create tag example
    match client.create_tag("rust-sdk-test").await {
        Ok(new_tag) => println!("New tag: {:?}", new_tag),
        Err(e) => eprintln!("Error creating tag: {}", e),
    }

    /// Get fields example
    match client.get_fields().await {
        Ok(fields) => println!("Fields: {:?}", fields),
        Err(e) => eprintln!("Error getting fields: {}", e),
    }

    /// Create field example
    match client.create_field("rust_test_field").await {
        Ok(new_field) => println!("New field: {:?}", new_field),
        Err(e) => eprintln!("Error creating field: {}", e),
    }

    /// Get stats examples
    match client.get_site_stats().await {
        Ok(site_stats) => println!("Site stats: {:?}", site_stats),
        Err(e) => eprintln!("Error getting site stats: {}", e),
    }

    Ok(())
}