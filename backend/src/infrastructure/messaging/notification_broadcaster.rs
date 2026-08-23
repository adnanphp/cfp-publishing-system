use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{Notification, NotificationPriority, NotificationType, MessagingError, MessageHandler};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BroadcastChannel {
    AllUsers,
    SpecificUsers(Vec<i32>),
    Channel(String),
    Role(String),
    Committee(i32),
}

#[derive(Debug, Clone)]
pub struct NotificationRule {
    pub id: Uuid,
    pub name: String,
    pub channel: BroadcastChannel,
    pub notification_type: NotificationType,
    pub priority_filter: Option<NotificationPriority>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NotificationQueueItem {
    pub notification: Notification,
    pub channel: BroadcastChannel,
    pub retry_count: u32,
    pub scheduled_for: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub struct NotificationBroadcaster {
    rules: Arc<RwLock<Vec<NotificationRule>>>,
    queue: Arc<Mutex<VecDeque<NotificationQueueItem>>>,
    delivery_history: Arc<RwLock<HashMap<String, DeliveryStatus>>>,
    handlers: Arc<RwLock<Vec<Arc<dyn MessageHandler>>>>,
}

#[derive(Debug, Clone)]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed(String),
    Retrying(u32),
}

impl NotificationBroadcaster {
    pub fn new() -> Self {
        NotificationBroadcaster {
            rules: Arc::new(RwLock::new(Vec::new())),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            delivery_history: Arc::new(RwLock::new(HashMap::new())),
            handlers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn initialize(&self) -> Result<(), MessagingError> {
        // Load rules from database or configuration
        self.load_default_rules().await?;
        
        // Start queue processor
        self.start_queue_processor();
        
        Ok(())
    }

    async fn load_default_rules(&self) -> Result<(), MessagingError> {
        let mut rules = self.rules.write().await;
        
        // Default rules based on notification types
        let default_rules = vec![
            NotificationRule {
                id: Uuid::new_v4(),
                name: "High Priority Urgent".to_string(),
                channel: BroadcastChannel::AllUsers,
                notification_type: NotificationType::SystemMaintenance,
                priority_filter: Some(NotificationPriority::Urgent),
                enabled: true,
                created_at: Utc::now(),
            },
            NotificationRule {
                id: Uuid::new_v4(),
                name: "Donation Notifications".to_string(),
                channel: BroadcastChannel::SpecificUsers(vec![]), // Will be populated dynamically
                notification_type: NotificationType::DonationReceived,
                priority_filter: None,
                enabled: true,
                created_at: Utc::now(),
            },
            NotificationRule {
                id: Uuid::new_v4(),
                name: "Plagiarism Alerts".to_string(),
                channel: BroadcastChannel::Channel("plagiarism".to_string()),
                notification_type: NotificationType::PlagiarismReported,
                priority_filter: Some(NotificationPriority::High),
                enabled: true,
                created_at: Utc::now(),
            },
            NotificationRule {
                id: Uuid::new_v4(),
                name: "Committee Updates".to_string(),
                channel: BroadcastChannel::Role("committee_member".to_string()),
                notification_type: NotificationType::CommitteeDecision,
                priority_filter: Some(NotificationPriority::Medium),
                enabled: true,
                created_at: Utc::now(),
            },
        ];
        
        rules.extend(default_rules);
        Ok(())
    }

    fn start_queue_processor(&self) {
        let queue = self.queue.clone();
        let handlers = self.handlers.clone();
        let delivery_history = self.delivery_history.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                
                let now = Utc::now();
                let mut items_to_process = Vec::new();
                
                // Get items ready for processing
                {
                    let mut queue = queue.lock().await;
                    while let Some(item) = queue.front() {
                        if item.scheduled_for <= now {
                            if let Some(item) = queue.pop_front() {
                                items_to_process.push(item);
                            }
                        } else {
                            break;
                        }
                    }
                }
                
                // Process each item
                for item in items_to_process {
                    let notification_id = item.notification.id.clone();
                    
                    // Check if expired
                    if item.expires_at <= now {
                        let mut history = delivery_history.write().await;
                        history.insert(notification_id, DeliveryStatus::Failed("Expired".to_string()));
                        continue;
                    }
                    
                    // Try to deliver
                    let delivery_result = Self::deliver_notification(&item, &handlers).await;
                    
                    let mut history = delivery_history.write().await;
                    
                    match delivery_result {
                        Ok(_) => {
                            history.insert(notification_id, DeliveryStatus::Delivered);
                        }
                        Err(e) => {
                            if item.retry_count < 3 {
                                // Reschedule for retry
                                let mut retry_item = item.clone();
                                retry_item.retry_count += 1;
                                retry_item.scheduled_for = Utc::now() + chrono::Duration::minutes(5);
                                
                                let mut queue = queue.lock().await;
                                queue.push_back(retry_item);
                                
                                history.insert(notification_id, DeliveryStatus::Retrying(item.retry_count + 1));
                            } else {
                                history.insert(notification_id, DeliveryStatus::Failed(e.to_string()));
                            }
                        }
                    }
                }
            }
        });
    }

    async fn deliver_notification(
        item: &NotificationQueueItem,
        handlers: &Arc<RwLock<Vec<Arc<dyn MessageHandler>>>>,
    ) -> Result<(), MessagingError> {
        let handlers = handlers.read().await;
        
        for handler in handlers.iter() {
            match handler.broadcast_notification(item.notification.clone()) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    log::warn!("Failed to deliver notification via handler: {}", e);
                    continue;
                }
            }
        }
        
        Err(MessagingError::RecipientNotFound)
    }

    pub async fn add_handler(&self, handler: Arc<dyn MessageHandler>) {
        let mut handlers = self.handlers.write().await;
        handlers.push(handler);
    }

    pub async fn broadcast(
        &self,
        notification: Notification,
        channel: Option<BroadcastChannel>,
    ) -> Result<String, MessagingError> {
        // Determine broadcast channel
        let broadcast_channel = match channel {
            Some(ch) => ch,
            None => {
                // Find matching rule
                let rules = self.rules.read().await;
                let rule = rules.iter()
                    .find(|r| r.notification_type == notification.notification_type && r.enabled)
                    .ok_or_else(|| MessagingError::RecipientNotFound)?;
                
                rule.channel.clone()
            }
        };

        // Create queue item
        let queue_item = NotificationQueueItem {
            notification: notification.clone(),
            channel: broadcast_channel,
            retry_count: 0,
            scheduled_for: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
        };

        // Add to queue
        let mut queue = self.queue.lock().await;
        queue.push_back(queue_item);

        // Record in history
        let mut history = self.delivery_history.write().await;
        history.insert(notification.id.clone(), DeliveryStatus::Pending);

        Ok(notification.id)
    }

    pub async fn schedule(
        &self,
        notification: Notification,
        channel: BroadcastChannel,
        scheduled_for: DateTime<Utc>,
    ) -> Result<String, MessagingError> {
        let queue_item = NotificationQueueItem {
            notification: notification.clone(),
            channel,
            retry_count: 0,
            scheduled_for,
            expires_at: scheduled_for + chrono::Duration::hours(24),
        };

        let mut queue = self.queue.lock().await;
        queue.push_back(queue_item);

        let mut history = self.delivery_history.write().await;
        history.insert(notification.id.clone(), DeliveryStatus::Pending);

        Ok(notification.id)
    }

    pub async fn get_delivery_status(&self, notification_id: &str) -> Option<DeliveryStatus> {
        let history = self.delivery_history.read().await;
        history.get(notification_id).cloned()
    }

    pub async fn add_rule(&self, rule: NotificationRule) -> Result<Uuid, MessagingError> {
        let mut rules = self.rules.write().await;
        rules.push(rule.clone());
        Ok(rule.id)
    }

    pub async fn update_rule(&self, rule_id: Uuid, enabled: bool) -> Result<(), MessagingError> {
        let mut rules = self.rules.write().await;
        if let Some(rule) = rules.iter_mut().find(|r| r.id == rule_id) {
            rule.enabled = enabled;
            Ok(())
        } else {
            Err(MessagingError::RecipientNotFound)
        }
    }

    pub async fn get_queue_stats(&self) -> QueueStats {
        let queue = self.queue.lock().await;
        let history = self.delivery_history.read().await;
        
        let pending = queue.len();
        let delivered = history.values()
            .filter(|&s| matches!(s, DeliveryStatus::Delivered))
            .count();
        let failed = history.values()
            .filter(|&s| matches!(s, DeliveryStatus::Failed(_)))
            .count();
        let retrying = history.values()
            .filter(|&s| matches!(s, DeliveryStatus::Retrying(_)))
            .count();

        QueueStats {
            pending,
            delivered,
            failed,
            retrying,
            total: history.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QueueStats {
    pub pending: usize,
    pub delivered: usize,
    pub failed: usize,
    pub retrying: usize,
    pub total: usize,
}

impl MessageHandler for NotificationBroadcaster {
    fn send_message(&self, _message: super::Message) -> Result<(), MessagingError> {
        // Broadcaster doesn't send individual messages
        Err(MessagingError::Unauthorized)
    }

    fn broadcast_notification(&self, notification: Notification) -> Result<(), MessagingError> {
        let broadcaster = self.clone();
        let notification_id = notification.id.clone();
        
        tokio::spawn(async move {
            match broadcaster.broadcast(notification, None).await {
                Ok(id) => log::info!("Broadcast notification {} successfully", id),
                Err(e) => log::error!("Failed to broadcast notification {}: {}", notification_id, e),
            }
        });

        Ok(())
    }

    fn get_user_messages(&self, _user_id: i32, _limit: Option<usize>) -> Vec<super::Message> {
        Vec::new()
    }

    fn mark_as_read(&self, _message_id: &str) -> Result<(), MessagingError> {
        Ok(())
    }
}

impl Clone for NotificationBroadcaster {
    fn clone(&self) -> Self {
        NotificationBroadcaster {
            rules: self.rules.clone(),
            queue: self.queue.clone(),
            delivery_history: self.delivery_history.clone(),
            handlers: self.handlers.clone(),
        }
    }
}
