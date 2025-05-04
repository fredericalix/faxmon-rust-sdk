# Faxmon SDK for Rust

A Rust SDK for interacting with the Faxmon API, allowing external monitoring agents or probes to send event status updates.

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
faxmon-sdk-rust = "0.1.0"
```

Alternatively, you can specify the repository directly:

```toml
[dependencies]
faxmon-sdk-rust = { git = "https://github.com/faxmon/faxmon-sdk-rust" }
```

## Features

- Simple API client for Faxmon backend
- Type-safe event status updates
- Asynchronous API using tokio
- Comprehensive error handling
- Full serialization/deserialization of API models using serde

## Usage

### Creating a Client

```rust
use faxmon_sdk_rust::Client;

let base_url = "https://faxmon.example.com"; // URL of your Faxmon backend
let probe_token = "your-probe-token"; // Token obtained from Faxmon

// Create a client
let client = Client::new(base_url, probe_token.to_string())
    .expect("Failed to create client");
```

### Sending an Event Status Update

```rust
use faxmon_sdk_rust::{EventStatus, EventStatusUpdatePayload};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a status update payload
    let payload = EventStatusUpdatePayload::new(EventStatus::Ok)
        .with_message("System is operating normally")
        .with_acknowledged(false);
        
    // Send the update
    let event_id = "your-event-id";
    let result = client.update_event_status(event_id, payload).await?;
    
    println!("Update successful: {:?}", result);
    Ok(())
}
```

### Error Handling

The SDK returns a custom `Error` type that wraps different error scenarios:

```rust
match client.update_event_status(event_id, payload).await {
    Ok(response) => {
        println!("Update successful: {:?}", response);
    },
    Err(err) => {
        match err {
            faxmon_sdk_rust::Error::Api { status, message } => {
                println!("API error ({}): {}", status, message);
            },
            faxmon_sdk_rust::Error::Request(req_err) => {
                println!("Request error: {}", req_err);
            },
            _ => println!("Other error: {}", err),
        }
    }
}
```

## Examples

See the `examples` directory for complete usage examples:

- `send_update.rs`: Demonstrates sending an event status update

Run an example with:

```bash
FAXMON_API_URL=https://faxmon.example.com \
FAXMON_PROBE_TOKEN=your-probe-token \
FAXMON_EVENT_ID=your-event-id \
cargo run --example send_update
```

## API Documentation

For detailed API documentation, run:

```bash
cargo doc --open
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
