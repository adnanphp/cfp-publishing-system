use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::{MultiPart, SinglePart, Attachment, header::ContentType};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::Path;
use mime::Mime;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::{ExternalServiceError, ServiceHealth, ServiceStatus, ExternalService, AsAny};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailServiceConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
    pub templates_dir: String,
    pub enable_tls: bool,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content: Vec<u8>,
    pub content_type: Mime,
    pub disposition: AttachmentDisposition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttachmentDisposition {
    Inline,
    Attachment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailRequest {
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: String,
    pub template: Option<String>,
    pub template_data: Option<serde_json::Value>,
    pub html_body: Option<String>,
    pub text_body: Option<String>,
    pub attachments: Vec<EmailAttachment>,
    pub reply_to: Option<EmailAddress>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailResponse {
    pub email_id: String,
    pub status: EmailStatus,
    pub message_id: Option<String>,
    pub sent_at: DateTime<Utc>,
    pub recipients: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmailStatus {
    Queued,
    Sent,
    Delivered,
    Failed,
    Bounced,
    Complained,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailWebhook {
    pub event_type: EmailWebhookEventType,
    pub email_id: String,
    pub recipient: String,
    pub status: EmailStatus,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailWebhookEventType {
    Sent,
    Delivered,
    Failed,
    Bounced,
    Opened,
    Clicked,
    Complained,
    Unsubscribed,
}

pub struct EmailService {
    config: EmailServiceConfig,
    transport: Option<SmtpTransport>,
    handlebars: Handlebars<'static>,
    initialized: bool,
}

impl EmailService {
    pub fn new(config: EmailServiceConfig) -> Self {
        EmailService {
            config,
            transport: None,
            handlebars: Handlebars::new(),
            initialized: false,
        }
    }

    fn build_message(&self, request: &EmailRequest) -> Result<Message, ExternalServiceError> {
        let from = format!("{} <{}>", self.config.from_name, self.config.from_email);
        let mut message_builder = Message::builder()
            .from(from.parse().map_err(|e| {
                ExternalServiceError::ValidationError(format!("Invalid from address: {}", e))
            })?)
            .subject(&request.subject);

        // Add recipients
        for to in &request.to {
            let address = match &to.name {
                Some(name) => format!("{} <{}>", name, to.email),
                None => to.email.clone(),
            };
            message_builder = message_builder.to(address.parse().map_err(|e| {
                ExternalServiceError::ValidationError(format!("Invalid to address: {}", e))
            })?);
        }

        // Add CC recipients
        for cc in &request.cc {
            let address = match &cc.name {
                Some(name) => format!("{} <{}>", name, cc.email),
                None => cc.email.clone(),
            };
            message_builder = message_builder.cc(address.parse().map_err(|e| {
                ExternalServiceError::ValidationError(format!("Invalid cc address: {}", e))
            })?);
        }

        // Add BCC recipients
        for bcc in &request.bcc {
            let address = match &bcc.name {
                Some(name) => format!("{} <{}>", name, bcc.email),
                None => bcc.email.clone(),
            };
            message_builder = message_builder.bcc(address.parse().map_err(|e| {
                ExternalServiceError::ValidationError(format!("Invalid bcc address: {}", e))
            })?);
        }

        // Add reply-to
        if let Some(reply_to) = &request.reply_to {
            let address = match &reply_to.name {
                Some(name) => format!("{} <{}>", name, reply_to.email),
                None => reply_to.email.clone(),
            };
            message_builder = message_builder.reply_to(address.parse().map_err(|e| {
                ExternalServiceError::ValidationError(format!("Invalid reply-to address: {}", e))
            })?);
        }

        // Build message body
        let mut multipart = MultiPart::alternative();

        // Add text part if available
        if let Some(text_body) = &request.text_body {
            let text_part = SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(text_body.clone());
            multipart = multipart.singlepart(text_part);
        } else if let (Some(template), Some(template_data)) = (&request.template, &request.template_data) {
            // Render text template
            let rendered = self.handlebars
                .render_template(&format!("{}.txt", template), &template_data)
                .map_err(|e| ExternalServiceError::ValidationError(e.to_string()))?;
            let text_part = SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(rendered);
            multipart = multipart.singlepart(text_part);
        }

        // Add HTML part if available
        if let Some(html_body) = &request.html_body {
            let html_part = SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html_body.clone());
            multipart = multipart.singlepart(html_part);
        } else if let (Some(template), Some(template_data)) = (&request.template, &request.template_data) {
            // Render HTML template
            let rendered = self.handlebars
                .render_template(&format!("{}.html", template), &template_data)
                .map_err(|e| ExternalServiceError::ValidationError(e.to_string()))?;
            let html_part = SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(rendered);
            multipart = multipart.singlepart(html_part);
        }

        // Add attachments
        for attachment in &request.attachments {
            let attachment_part = Attachment::new(attachment.filename.clone())
                .body(attachment.content.clone(), attachment.content_type.clone())
                .disposition(match attachment.disposition {
                    lettre::message::SinglePart::inline => lettre::message::SinglePart::Inline,
                    lettre::message::SinglePart::attachment => lettre::message::SinglePart::Attachment,
                });
            multipart = multipart.singlepart(attachment_part);
        }

        let message = message_builder
            .multipart(multipart)
            .map_err(|e| ExternalServiceError::ValidationError(e.to_string()))?;

        Ok(message)
    }

    pub async fn send_email(&self, request: EmailRequest) -> Result<EmailResponse, ExternalServiceError> {
        if !self.initialized {
            return Err(ExternalServiceError::ServiceUnavailable("Email service not initialized".to_string()));
        }

        let transport = self.transport.as_ref()
            .ok_or_else(|| ExternalServiceError::ServiceUnavailable("Email transport not available".to_string()))?;

        // Build message
        let message = self.build_message(&request)?;

        // Send email
        let result = transport.send(&message);

        match result {
            Ok(response) => {
                let email_id = Uuid::new_v4().to_string();
                let recipients: Vec<String> = request.to.iter()
                    .map(|addr| addr.email.clone())
                    .collect();

                Ok(EmailResponse {
                    email_id,
                    status: EmailStatus::Sent,
                    message_id: response.message_id().map(|s| s.to_string()),
                    sent_at: Utc::now(),
                    recipients,
                })
            }
            Err(e) => {
                Err(ExternalServiceError::EmailError(format!("Failed to send email: {}", e)))
            }
        }
    }

    pub async fn send_bulk_emails(&self, requests: Vec<EmailRequest>) -> Vec<Result<EmailResponse, ExternalServiceError>> {
        let mut results = Vec::new();
        
        for request in requests {
            let result = self.send_email(request).await;
            results.push(result);
            
            // Small delay to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        results
    }

    pub fn load_template(&mut self, name: &str, content: &str) -> Result<(), ExternalServiceError> {
        self.handlebars
            .register_template_string(name, content)
            .map_err(|e| ExternalServiceError::ValidationError(e.to_string()))
    }

    pub fn load_template_file(&mut self, name: &str, path: &Path) -> Result<(), ExternalServiceError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ExternalServiceError::ValidationError(e.to_string()))?;
        
        self.handlebars
            .register_template_string(name, content)
            .map_err(|e| ExternalServiceError::ValidationError(e.to_string()))
    }

    pub fn render_template(&self, name: &str, data: &serde_json::Value) -> Result<String, ExternalServiceError> {
        self.handlebars
            .render(name, data)
            .map_err(|e| ExternalServiceError::ValidationError(e.to_string()))
    }

    // Common email templates for CFP system
    pub async fn send_welcome_email(&self, to: EmailAddress, member_name: &str) -> Result<EmailResponse, ExternalServiceError> {
        let request = EmailRequest {
            to: vec![to],
            cc: vec![],
            bcc: vec![],
            subject: format!("Welcome to CopyForward Publishing, {}!", member_name),
            template: Some("welcome".to_string()),
            template_data: Some(serde_json::json!({
                "member_name": member_name,
                "current_year": Utc::now().year(),
            })),
            html_body: None,
            text_body: None,
            attachments: vec![],
            reply_to: None,
            tags: vec!["welcome".to_string(), "onboarding".to_string()],
            metadata: HashMap::new(),
        };

        self.send_email(request).await
    }

    pub async fn send_donation_confirmation(
        &self,
        to: EmailAddress,
        donation_amount: f64,
        currency: &str,
        text_title: &str,
        charity_name: &str,
    ) -> Result<EmailResponse, ExternalServiceError> {
        let request = EmailRequest {
            to: vec![to],
            cc: vec![],
            bcc: vec![],
            subject: format!("Donation Confirmation - {}", text_title),
            template: Some("donation_confirmation".to_string()),
            template_data: Some(serde_json::json!({
                "donation_amount": donation_amount,
                "currency": currency,
                "text_title": text_title,
                "charity_name": charity_name,
                "date": Utc::now().format("%Y-%m-%d").to_string(),
            })),
            html_body: None,
            text_body: None,
            attachments: vec![],
            reply_to: None,
            tags: vec!["donation".to_string(), "confirmation".to_string()],
            metadata: HashMap::new(),
        };

        self.send_email(request).await
    }

    pub async fn send_plagiarism_alert(
        &self,
        to: EmailAddress,
        case_id: i32,
        text_title: &str,
        reporter_name: &str,
    ) -> Result<EmailResponse, ExternalServiceError> {
        let request = EmailRequest {
            to: vec![to],
            cc: vec![],
            bcc: vec![],
            subject: format!("Plagiarism Alert - Case #{}", case_id),
            template: Some("plagiarism_alert".to_string()),
            template_data: Some(serde_json::json!({
                "case_id": case_id,
                "text_title": text_title,
                "reporter_name": reporter_name,
                "date": Utc::now().format("%Y-%m-%d").to_string(),
                "appeal_deadline": (Utc::now() + chrono::Duration::days(14)).format("%Y-%m-%d").to_string(),
            })),
            html_body: None,
            text_body: None,
            attachments: vec![],
            reply_to: None,
            tags: vec!["plagiarism".to_string(), "alert".to_string()],
            metadata: HashMap::new(),
        };

        self.send_email(request).await
    }

    pub async fn send_committee_invitation(
        &self,
        to: EmailAddress,
        committee_name: &str,
        role: &str,
        invite_token: &str,
    ) -> Result<EmailResponse, ExternalServiceError> {
        let request = EmailRequest {
            to: vec![to],
            cc: vec![],
            bcc: vec![],
            subject: format!("Invitation to Join {} Committee", committee_name),
            template: Some("committee_invitation".to_string()),
            template_data: Some(serde_json::json!({
                "committee_name": committee_name,
                "role": role,
                "invite_token": invite_token,
                "expiry_date": (Utc::now() + chrono::Duration::days(7)).format("%Y-%m-%d").to_string(),
            })),
            html_body: None,
            text_body: None,
            attachments: vec![],
            reply_to: None,
            tags: vec!["committee".to_string(), "invitation".to_string()],
            metadata: HashMap::new(),
        };

        self.send_email(request).await
    }
}

#[async_trait]
impl ExternalService for EmailService {
    async fn initialize(&mut self) -> Result<(), ExternalServiceError> {
        // Create SMTP transport
        let creds = Credentials::new(
            self.config.smtp_username.clone(),
            self.config.smtp_password.clone(),
        );

        let mut transport_builder = SmtpTransport::relay(&self.config.smtp_host)
            .map_err(|e| ExternalServiceError::ConfigurationError(e.to_string()))?
            .port(self.config.smtp_port)
            .credentials(creds);

        if self.config.enable_tls {
            transport_builder = transport_builder.tls(lettre::transport::smtp::client::TlsParameters::default());
        }

        let transport = transport_builder.build();

        // Test connection
        transport.test_connection()
            .map_err(|e| ExternalServiceError::NetworkError(e.to_string()))?;

        self.transport = Some(transport);

        // Load email templates
        let templates_dir = Path::new(&self.config.templates_dir);
        if templates_dir.exists() {
            self.load_templates_from_dir(templates_dir).await?;
        } else {
            // Load default templates
            self.load_default_templates().await?;
        }

        self.initialized = true;
        Ok(())
    }

    async fn health_check(&self) -> Result<ServiceHealth, ExternalServiceError> {
        let start_time = std::time::Instant::now();
        
        let transport = self.transport.as_ref()
            .ok_or_else(|| ExternalServiceError::ServiceUnavailable("Email transport not available".to_string()))?;

        let result = transport.test_connection();
        let response_time = start_time.elapsed().as_secs_f64() * 1000.0;

        match result {
            Ok(_) => Ok(ServiceHealth {
                service: "Email Service".to_string(),
                status: ServiceStatus::Healthy,
                response_time,
                last_checked: Utc::now(),
                details: serde_json::json!({
                    "status": "healthy",
                    "templates_loaded": self.handlebars.get_templates().len()
                }),
            }),
            Err(e) => Ok(ServiceHealth {
                service: "Email Service".to_string(),
                status: ServiceStatus::Unhealthy,
                response_time,
                last_checked: Utc::now(),
                details: serde_json::json!({
                    "status": "unhealthy",
                    "error": e.to_string()
                }),
            }),
        }
    }

    fn get_name(&self) -> &str {
        "Email Service"
    }
}

impl EmailService {
    async fn load_templates_from_dir(&mut self, dir: &Path) -> Result<(), ExternalServiceError> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| ExternalServiceError::ConfigurationError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| ExternalServiceError::ConfigurationError(e.to_string()))?;
            let path = entry.path();
            
            if path.is_file() {
                let filename = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                
                self.load_template_file(filename, &path)?;
            }
        }

        Ok(())
    }

    async fn load_default_templates(&mut self) -> Result<(), ExternalServiceError> {
        // Welcome template
        let welcome_template = include_str!("../../../templates/welcome.html");
        self.load_template("welcome", welcome_template)?;

        // Donation confirmation template
        let donation_template = include_str!("../../../templates/donation_confirmation.html");
        self.load_template("donation_confirmation", donation_template)?;

        // Plagiarism alert template
        let plagiarism_template = include_str!("../../../templates/plagiarism_alert.html");
        self.load_template("plagiarism_alert", plagiarism_template)?;

        // Committee invitation template
        let committee_template = include_str!("../../../templates/committee_invitation.html");
        self.load_template("committee_invitation", committee_template)?;

        Ok(())
    }
}

impl AsAny for EmailService {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
