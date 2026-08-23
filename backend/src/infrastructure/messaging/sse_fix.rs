use actix_web::{Error, HttpRequest, HttpResponse};
use futures::stream::Stream;
use std::pin::Pin;
use uuid::Uuid;

pub struct SseEvent {
    pub data: String,
    pub event: Option<String>,
    pub id: Option<String>,
    pub retry: Option<u64>,
}

impl SseEvent {
    pub fn new(data: String) -> Self {
        Self {
            data,
            event: None,
            id: None,
            retry: None,
        }
    }
    
    pub fn with_event(mut self, event: String) -> Self {
        self.event = Some(event);
        self
    }
    
    pub fn format(&self) -> String {
        let mut result = String::new();
        
        if let Some(event) = &self.event {
            result.push_str(&format!("event: {}\n", event));
        }
        
        if let Some(id) = &self.id {
            result.push_str(&format!("id: {}\n", id));
        }
        
        if let Some(retry) = self.retry {
            result.push_str(&format!("retry: {}\n", retry));
        }
        
        // Split data by lines for proper SSE formatting
        for line in self.data.lines() {
            result.push_str(&format!("data: {}\n", line));
        }
        
        result.push_str("\n");
        result
    }
}

pub type SseStream = Pin<Box<dyn Stream<Item = Result<SseEvent, Error>> + Send>>;
