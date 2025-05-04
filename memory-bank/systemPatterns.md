# System Patterns: faxmon-sdk-rust

## Coding Patterns
- **Error Handling**: Use Result<T, Error> with custom Error enum from thiserror
- **Async/Await**: All network operations are async using tokio
- **Builder Pattern**: For constructing complex objects with optional fields
- **Documentation**: All public interfaces have rustdoc comments

## Architectural Patterns
- **Module Organization**: Clear separation between client, models, and error handling
- **HTTP Client**: Single reusable client instance for better performance
- **Type Safety**: Strong typing for all API interactions
- **Optional Fields**: Use Option<T> for all optional fields in API models

## Testing Patterns
- Unit tests for internal logic
- Integration tests for API interaction (to be mocked)

[2025-05-04 09:18:35] - Initial patterns documentation
