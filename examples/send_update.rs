// send_update.rs - Example of using the faxmon-sdk-rust to send an event status update
//
// This example demonstrates how to create a client and send an event status update
// to the Faxmon API.

use faxmon_sdk_rust::{Client, EventStatus, EventStatusUpdatePayload};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get environment variables or use defaults for demonstration
    let base_url = env::var("FAXMON_API_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let probe_token = env::var("FAXMON_PROBE_TOKEN").expect("FAXMON_PROBE_TOKEN must be set");
    let event_id = env::var("FAXMON_EVENT_ID").expect("FAXMON_EVENT_ID must be set");
    
    // Create a client
    println!("Creating Faxmon client for {}", base_url);
    let client = Client::new(&base_url, probe_token)?;
    
    // Create a status update payload
    let payload = EventStatusUpdatePayload::new(EventStatus::Ok)
        .with_message("System is operating normally")
        .with_acknowledged(false);
    
    // Send the update
    println!("Sending status update for event: {}", event_id);
    match client.update_event_status(&event_id, payload).await {
        Ok(response) => {
            println!("Update successful!");
            println!("Event ID: {}", response.id);
            println!("Event Name: {}", response.name);
            println!("Current Status: {:?}", response.status);
            println!("Acknowledged: {}", response.acknowledged);
            println!("Last Updated: {}", response.last_updated);
            println!("Message: {}", response.message);
        },
        Err(err) => {
            println!("Error updating event status: {}", err);
            if let Some(status) = err.status_code() {
                println!("HTTP Status: {}", status);
            }
        }
    }
    
    Ok(())
}
