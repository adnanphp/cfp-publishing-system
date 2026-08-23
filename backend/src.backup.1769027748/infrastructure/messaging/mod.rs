pub mod websocket_handler;
pub mod sse_handler;
pub mod notification_broadcaster;

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Notification,
    SystemAlert,
    DonationUpdate,
    PlagiarismAlert,
    CommitteeUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub message_type: MessageType,
    pub sender_id: Option<i32>,
    pub recipient_id: i32,
    pub subject: Option<String>,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub notification_type: NotificationType,
    pub user_id: i32,
    pub title: String,
    pub message: String,
    pub priority: NotificationPriority,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    DonationReceived,
    DonationDistributed,
    PlagiarismReported,
    PlagiarismVerdict,
    CommitteeInvitation,
    CommitteeDecision,
    TextPublished,
    CommentReceived,
    SystemMaintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug)]
pub enum MessagingError {
    ConnectionFailed,
    SerializationError,
    DeserializationError,
    ChannelFull,
    Timeout,
    RecipientNotFound,
    Unauthorized,
}

impl std::fmt::Display for MessagingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessagingError::ConnectionFailed => write!(f, "Failed to establish connection"),
            MessagingError::SerializationError => write!(f, "Failed to serialize message"),
            MessagingError::DeserializationError => write!(f, "Failed to deserialize message"),
            MessagingError::ChannelFull => write!(f, "Message channel is full"),
            MessagingError::Timeout => write!(f, "Message delivery timeout"),
            MessagingError::RecipientNotFound => write!(f, "Recipient not found"),
            MessagingError::Unauthorized => write!(f, "Unauthorized to send message"),
        }
    }
}

impl std::error::Error for MessagingError {}

pub trait MessageHandler: Send + Sync {
    fn send_message(&self, message: Message) -> Result<(), MessagingError>;
    fn broadcast_notification(&self, notification: Notification) -> Result<(), MessagingError>;
    fn get_user_messages(&self, user_id: i32, limit: Option<usize>) -> Vec<Message>;
    fn mark_as_read(&self, message_id: &str) -> Result<(), MessagingError>;
}

pub struct MessagingManager {
    handlers: Vec<Arc<dyn MessageHandler>>,
    active_connections: Arc<RwLock<HashMap<i32, Vec<String>>>>,
}

impl MessagingManager {
    pub fn new() -> Self {
        MessagingManager {
            handlers: Vec::new(),
            active_connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_handler(&mut self, handler: Arc<dyn MessageHandler>) {
        self.handlers.push(handler);
    }

    pub async fn send_message(&self, message: Message) -> Result<(), MessagingError> {
        for handler in &self.handlers {
            if let Err(e) = handler.send_message(message.clone()) {
                log::error!("Failed to send message via handler: {}", e);
                continue;
            }
        }
        Ok(())
    }

    pub async fn broadcast_notification(&self, notification: Notification) -> Result<(), MessagingError> {
        for handler in &self.handlers {
            if let Err(e) = handler.broadcast_notification(notification.clone()) {
                log::error!("Failed to broadcast notification via handler: {}", e);
                continue;
            }
        }
        Ok(())
    }

    pub async fn register_connection(&self, user_id: i32, connection_id: String) {
        let mut connections = self.active_connections.write().await;
        connections.entry(user_id).or_insert_with(Vec::new).push(connection_id);
    }

    pub async fn unregister_connection(&self, user_id: i32, connection_id: &str) {
        let mut connections = self.active_connections.write().await;
        if let Some(user_connections) = connections.get_mut(&user_id) {
            user_connections.retain(|id| id != connection_id);
            if user_connections.is_empty() {
                connections.remove(&user_id);
            }
        }
    }

    pub async fn get_active_users(&self) -> Vec<i32> {
        let connections = self.active_connections.read().await;
        connections.keys().cloned().collect()
    }
}

// Message broker for different notification channels
pub struct MessageBroker {
    pub websocket: websocket_handler::WebSocketServer,
    pub sse: sse_handler::SseHandler,
    pub broadcaster: notification_broadcaster::NotificationBroadcaster,
}

impl MessageBroker {
    pub fn new() -> Self {
        MessageBroker {
            websocket: websocket_handler::WebSocketServer::new(),
            sse: sse_handler::SseHandler::new(),
            broadcaster: notification_broadcaster::NotificationBroadcaster::new(),
        }
    }

    pub async fn initialize(&self) -> Result<(), MessagingError> {
        // Initialize all handlers
        self.websocket.initialize().await?;
        self.sse.initialize().await?;
        self.broadcaster.initialize().await?;
        
        Ok(())
    }
}
