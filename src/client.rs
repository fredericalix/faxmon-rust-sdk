// client.rs - Faxmon API client implementation
//
// This module provides the main Client struct for interacting with the Faxmon API.

use crate::{
    error::Error,
    models::{EventConsolidatedState, EventStatusUpdatePayload, ErrorResponse},
    Result,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client as ReqwestClient;
use url::Url;

/// Client for interacting with the Faxmon API
///
/// This client handles authentication and provides methods for interacting
/// with the Faxmon API endpoints.
#[derive(Debug, Clone)]
pub struct Client {
    /// Base URL for the Faxmon API
    base_url: Url,
    
    /// Probe token for authentication
    probe_token: String,
    
    /// HTTP client for making requests
    http_client: ReqwestClient,
}

impl Client {
    /// Creates a new Faxmon API client
    ///
    /// # Arguments
    ///
    /// * `base_url_str` - Base URL of the Faxmon API
    /// * `probe_token` - Probe token for authentication
    ///
    /// # Returns
    ///
    /// A new `Client` or an error if the base URL cannot be parsed
    ///
    /// # Example
    ///
    /// ```
    /// use faxmon_sdk_rust::Client;
    ///
    /// let client = Client::new("https://faxmon.example.com", "my-probe-token".to_string())
    ///     .expect("Failed to create client");
    /// ```
    pub fn new(base_url_str: &str, probe_token: String) -> Result<Self> {
        // Parse and validate the base URL
        let base_url = Url::parse(base_url_str)
            .map_err(Error::UrlParse)?;
        
        // Create the HTTP client with default configuration
        let http_client = ReqwestClient::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(Error::Request)?;
        
        Ok(Self {
            base_url,
            probe_token,
            http_client,
        })
    }
    
    /// Creates a new Faxmon API client with a custom HTTP client
    ///
    /// # Arguments
    ///
    /// * `base_url_str` - Base URL of the Faxmon API
    /// * `probe_token` - Probe token for authentication
    /// * `http_client` - Custom HTTP client
    ///
    /// # Returns
    ///
    /// A new `Client` or an error if the base URL cannot be parsed
    pub fn with_http_client(
        base_url_str: &str,
        probe_token: String,
        http_client: ReqwestClient,
    ) -> Result<Self> {
        let base_url = Url::parse(base_url_str)
            .map_err(Error::UrlParse)?;
        
        Ok(Self {
            base_url,
            probe_token,
            http_client,
        })
    }
    
    /// Updates the status of an event
    ///
    /// # Arguments
    ///
    /// * `event_id` - ID of the event to update
    /// * `payload` - Status update payload
    ///
    /// # Returns
    ///
    /// The updated consolidated state of the event or an error
    ///
    /// # Example
    ///
    /// ```
    /// use faxmon_sdk_rust::{Client, EventStatus, EventStatusUpdatePayload};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://faxmon.example.com", "my-probe-token".to_string())?;
    ///     
    ///     let payload = EventStatusUpdatePayload::new(EventStatus::Ok)
    ///         .with_message("System is operating normally");
    ///     
    ///     let result = client.update_event_status("event-id", payload).await?;
    ///     println!("Event updated: {:?}", result);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_event_status(
        &self,
        event_id: &str,
        payload: EventStatusUpdatePayload,
    ) -> Result<EventConsolidatedState> {
        // Construct the URL
        let url = self.base_url.join(&format!("events/{}/update", event_id))
            .map_err(Error::UrlParse)?;
        
        // Prepare headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.probe_token))
                .map_err(|e| Error::validation(format!("Invalid token: {}", e)))?,
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        
        // Send the request
        let response = self.http_client
            .post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .map_err(Error::Request)?;
        
        // Handle the response
        let status = response.status();
        
        if status.is_success() {
            // Parse the successful response
            let event_state = response.json::<EventConsolidatedState>().await
                .map_err(|e| {
                    Error::UnexpectedResponse(format!(
                        "Failed to parse successful response: {}", e
                    ))
                })?;
            
            Ok(event_state)
        } else {
            // Try to parse an error response
            let error_text = response.text().await
                .unwrap_or_else(|_| String::from("Unknown error"));
            
            // Try to parse as an ErrorResponse
            let error_message = serde_json::from_str::<ErrorResponse>(&error_text)
                .map(|er| er.error)
                .unwrap_or(error_text);
            
            Err(Error::api(status, error_message))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::EventStatus;
    
    #[test]
    fn test_client_new() {
        let client = Client::new("https://faxmon.example.com", "token".to_string());
        assert!(client.is_ok());
        
        let client = Client::new("invalid-url", "token".to_string());
        assert!(client.is_err());
    }
}
