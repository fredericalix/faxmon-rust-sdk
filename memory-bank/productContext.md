# Product Context: faxmon-sdk-rust

## Project Overview
`faxmon-sdk-rust` is a Rust SDK (crate) designed to interact with the Faxmon API backend. It simplifies the process of sending event status updates from external monitoring agents or probes.

## Components
- **Client**: Main interface for interacting with the Faxmon API
- **Models**: Data structures for API requests and responses
- **Error Handling**: Custom error types for SDK operations

## Organization
The project follows standard Rust library structure:
- `src/lib.rs`: Main entry point and exports
- `src/client.rs`: API client implementation
- `src/models.rs`: Data structures and serialization
- `src/error.rs`: Error handling

## Standards
- Rust Edition: 2021 or newer
- Complete documentation (Rustdoc)
- Error handling using thiserror
- Async/await pattern with tokio runtime
- JSON serialization/deserialization using serde

[2025-05-04 09:18:35] - Initial project definition
