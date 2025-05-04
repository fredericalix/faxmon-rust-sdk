// models.rs - Data models for the Faxmon API
//
// This module defines the data structures used for communication with the Faxmon API.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Status of an event in the Faxmon system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    /// System is operating normally
    Ok,
    /// System is operating with warnings
    Warning,
    /// System is in a critical state
    Critical,
    /// System state is unknown
    Unknown,
}

/// Payload for updating the status of an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStatusUpdatePayload {
    /// The current status of the event
    pub status: EventStatus,
    
    /// A descriptive message associated with the current status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    
    /// Indicates if the current status should be considered acknowledged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<bool>,
    
    /// The timestamp when the status was recorded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    
    /// Specifies a particular notification listener to use for this update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_listener_id: Option<String>,
}

impl EventStatusUpdatePayload {
    /// Creates a new EventStatusUpdatePayload with the specified status
    ///
    /// # Arguments
    ///
    /// * `status` - The current status of the event
    ///
    /// # Returns
    ///
    /// A new EventStatusUpdatePayload with the specified status and all optional fields set to None
    pub fn new(status: EventStatus) -> Self {
        Self {
            status,
            message: None,
            acknowledged: None,
            timestamp: None,
            notification_listener_id: None,
        }
    }
    
    /// Sets the message for this payload
    ///
    /// # Arguments
    ///
    /// * `message` - The message to set
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = Some(message.into());
        self
    }
    
    /// Sets the acknowledged flag for this payload
    ///
    /// # Arguments
    ///
    /// * `acknowledged` - The acknowledged flag to set
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_acknowledged(mut self, acknowledged: bool) -> Self {
        self.acknowledged = Some(acknowledged);
        self
    }
    
    /// Sets the timestamp for this payload
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The timestamp to set
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    /// Sets the notification listener ID for this payload
    ///
    /// # Arguments
    ///
    /// * `notification_listener_id` - The notification listener ID to set
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_notification_listener_id<S: Into<String>>(mut self, notification_listener_id: S) -> Self {
        self.notification_listener_id = Some(notification_listener_id.into());
        self
    }
}

/// Represents the consolidated state of an event in the Faxmon system
#[derive(Debug, Clone, Deserialize)]
pub struct EventConsolidatedState {
    /// The unique identifier of the event
    pub id: String,
    
    /// The name of the event
    pub name: String,
    
    /// The type of the event
    pub r#type: String,
    
    /// The designation of the event
    pub designation: String,
    
    /// The current status of the event
    pub status: EventStatus,
    
    /// Indicates if the current status is acknowledged
    pub acknowledged: bool,
    
    /// The timestamp when the event was last updated
    pub last_updated: DateTime<Utc>,
    
    /// A message associated with the current status
    pub message: String,
    
    /// The notification listener ID associated with the event
    #[serde(default)]
    pub notification_listener_id: Option<String>,
}

/// Represents an error response from the Faxmon API
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// The error message
    pub error: String,
}
