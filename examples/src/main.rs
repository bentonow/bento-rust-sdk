use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

use bento::{
    Client, ConfigBuilder,
    EventData,
    BroadcastData, BroadcastType, ContactData,
    subscriber::SubscriberInput,
};

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


    // Find subscriber example
    // match client.find_subscriber("test@example.com").await {
    //     Ok(subscriber) => println!("Found subscriber: {:?}", subscriber),
    //     Err(e) => eprintln!("Error finding subscriber: {}", e),
    // }

    // // Create subscriber example
    // let mut fields = HashMap::new();
    // fields.insert("company".to_string(), serde_json::json!("Acme Inc"));
    //
    // let input = SubscriberInput {
    //     email: "new@example.com".to_string(),
    //     first_name: Some("John".to_string()),
    //     last_name: Some("Doe".to_string()),
    //     tags: Some(vec!["new-user".to_string()]),
    //     fields: Some(fields),
    //     remove_tags: None,
    // };
    //
    // match client.create_subscriber(input).await {
    //     Ok(new_subscriber) => println!("Created subscriber: {:?}", new_subscriber),
    //     Err(e) => eprintln!("Error creating subscriber: {}", e),
    // }
    //
    // Track event example
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
    //
    // // Create broadcast example
    // let broadcast = BroadcastData {
    //     name: "Test Campaign".to_string(),
    //     subject: "Test Broadcast".to_string(),
    //     content: "<p>Hello subscribers!</p>".to_string(),
    //     broadcast_type: BroadcastType::Plain,
    //     from: ContactData {
    //         name: Some("John Doe".to_string()),
    //         email: "sender@yourdomain.com".to_string(),
    //     },
    //     inclusive_tags: None,
    //     exclusive_tags: None,
    //     segment_id: None,
    //     batch_size_per_hour: 1000,
    // };
    //
    // match client.create_broadcasts(vec![broadcast]).await {
    //     Ok(_) => println!("Successfully created broadcast"),
    //     Err(e) => eprintln!("Error creating broadcast: {}", e),
    // }
    //
    // Get tags example
    // match client.get_tags().await {
    //     Ok(tags) => println!("Tags: {:?}", tags),
    //     Err(e) => eprintln!("Error getting tags: {}", e),
    // }
    //
    // // Create tag example
    // match client.create_tag("rust-sdk-test").await {
    //     Ok(new_tag) => println!("New tag: {:?}", new_tag),
    //     Err(e) => eprintln!("Error creating tag: {}", e),
    // }
    //
    // Get fields example
    // match client.get_fields().await {
    //     Ok(fields) => println!("Fields: {:?}", fields),
    //     Err(e) => eprintln!("Error getting fields: {}", e),
    // }
    //
    // // Create field example
    // match client.create_field("test_field").await {
    //     Ok(new_field) => println!("New field: {:?}", new_field),
    //     Err(e) => eprintln!("Error creating field: {}", e),
    // }
    //
    // // Get stats examples
    // match client.get_site_stats().await {
    //     Ok(site_stats) => println!("Site stats: {:?}", site_stats),
    //     Err(e) => eprintln!("Error getting site stats: {}", e),
    // }

    Ok(())
}