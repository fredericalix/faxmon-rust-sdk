// faxmon-sdk-rust - A Rust SDK for interacting with the Faxmon API
//
// This SDK provides a simple interface for external monitoring agents or probes
// to interact with the Faxmon backend API for sending event status updates.

// Declare modules
mod client;
mod models;
mod error;

// Re-export public API
pub use client::Client;
pub use models::*;
pub use error::Error;

/// The type alias for results returned by this SDK's functions.
pub type Result<T> = std::result::Result<T, Error>;
