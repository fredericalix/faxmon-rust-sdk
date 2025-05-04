// error.rs - Error types for the Faxmon SDK
//
// This module defines the error types used throughout the SDK.

use std::fmt;
use thiserror::Error;
use reqwest::StatusCode;

/// Custom error type for the Faxmon SDK
#[derive(Error, Debug)]
pub enum Error {
    /// Error occurred during HTTP request
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),
    
    /// Error occurred while parsing a URL
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
    
    /// Error occurred while serializing/deserializing JSON
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Error returned by the Faxmon API
    #[error("API error (status {status}): {message}")]
    Api {
        /// HTTP status code
        status: StatusCode,
        
        /// Error message from the API
        message: String,
    },
    
    /// Error occurred due to an unexpected response format
    #[error("Unexpected response format: {0}")]
    UnexpectedResponse(String),
    
    /// Error occurred during input validation
    #[error("Validation error: {0}")]
    Validation(String),
}

impl Error {
    /// Creates a new API error from a status code and message
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code
    /// * `message` - The error message
    ///
    /// # Returns
    ///
    /// A new Error::Api instance
    pub fn api(status: StatusCode, message: impl fmt::Display) -> Self {
        Self::Api {
            status,
            message: message.to_string(),
        }
    }
    
    /// Creates a new validation error
    ///
    /// # Arguments
    ///
    /// * `message` - The validation error message
    ///
    /// # Returns
    ///
    /// A new Error::Validation instance
    pub fn validation(message: impl fmt::Display) -> Self {
        Self::Validation(message.to_string())
    }
    
    /// Returns the HTTP status code if this is an API error
    pub fn status_code(&self) -> Option<StatusCode> {
        match self {
            Self::Api { status, .. } => Some(*status),
            Self::Request(req_err) => req_err.status(),
            _ => None,
        }
    }
    
    /// Returns true if this is a 4xx client error
    pub fn is_client_error(&self) -> bool {
        self.status_code()
            .map(|status| status.is_client_error())
            .unwrap_or(false)
    }
    
    /// Returns true if this is a 5xx server error
    pub fn is_server_error(&self) -> bool {
        self.status_code()
            .map(|status| status.is_server_error())
            .unwrap_or(false)
    }
}
