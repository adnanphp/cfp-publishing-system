//! Comprehensive logging utilities for the CFP backend
//!
//! This module provides structured logging, request logging,
//! performance monitoring, and audit logging capabilities.

use std::io;
use std::sync::Once;
use serde_json::json;
use chrono::{DateTime, Utc};
use actix_web::middleware::Logger as ActixLogger;
use env_logger::{Builder, Target};
use log::{LevelFilter, Record, Level};
use fern::Dispatch;

/// Logging configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "off" => LogLevel::Off,
            "error" => LogLevel::Error,
            "warn" => LogLevel::Warn,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _ => LogLevel::Info,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LogConfig {
    pub level: LogLevel,
    pub enable_console: bool,
    pub enable_file: bool,
    pub enable_json: bool,
    pub file_path: Option<String>,
    pub max_file_size: u64,
    pub max_files: u32,
    pub enable_rotation: bool,
    pub include_timestamp: bool,
    pub include_module: bool,
    pub include_line_number: bool,
    pub enable_colors: bool,
    pub log_format: LogFormat,
}

/// Log format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Default,
    Compact,
    Pretty,
    Json,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            level: LogLevel::Info,
            enable_console: true,
            enable_file: false,
            enable_json: false,
            file_path: None,
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
            enable_rotation: true,
            include_timestamp: true,
            include_module: true,
            include_line_number: true,
            enable_colors: true,
            log_format: LogFormat::Default,
        }
    }
}

/// Structured log entry
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub target: String,
    pub message: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub thread_id: String,
    pub process_id: u32,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub correlation_id: Option<String>,
    pub additional_fields: serde_json::Value,
}

impl LogEntry {
    pub fn new(record: &Record) -> Self {
        LogEntry {
            timestamp: Utc::now(),
            level: record.level().to_string(),
            target: record.target().to_string(),
            message: record.args().to_string(),
            module_path: record.module_path().map(|s| s.to_string()),
            file: record.file().map(|s| s.to_string()),
            line: record.line(),
            thread_id: format!("{:?}", std::thread::current().id()),
            process_id: std::process::id(),
            request_id: None,
            user_id: None,
            correlation_id: None,
            additional_fields: serde_json::json!({}),
        }
    }
    
    pub fn with_request_id(mut self, request_id: &str) -> Self {
        self.request_id = Some(request_id.to_string());
        self
    }
    
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }
    
    pub fn with_correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_string());
        self
    }
    
    pub fn with_field<S: serde::Serialize>(mut self, key: &str, value: S) -> Self {
        self.additional_fields[key] = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
        self
    }
    
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
    
    pub fn to_plain_string(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(request_id) = &self.request_id {
            parts.push(format!("[{}]", request_id));
        }
        
        parts.push(format!("{}", self.timestamp.format("%Y-%m-%d %H:%M:%S%.3f")));
        parts.push(format!("{:5}", self.level.to_uppercase()));
        parts.push(format!("[{}]", self.thread_id));
        
        if let Some(module_path) = &self.module_path {
            parts.push(format!("{}:", module_path));
        }
        
        parts.push(self.message.clone());
        
        parts.join(" ")
    }
}

/// Initialize the logger with configuration
pub fn setup_logger(config: &crate::utils::UtilsConfig) -> Result<(), Box<dyn std::error::Error>> {
    static INIT: Once = Once::new();
    
    let mut builder = Builder::new();
    
    // Set log level
    builder.filter_level(LevelFilter::from(config.log_level));
    
    // Configure format based on environment
    match std::env::var("RUST_LOG_FORMAT").unwrap_or_default().as_str() {
        "json" => configure_json_format(&mut builder),
        "compact" => configure_compact_format(&mut builder),
        "pretty" => configure_pretty_format(&mut builder),
        _ => configure_default_format(&mut builder, &config),
    }
    
    // Write to stdout by default
    builder.target(Target::Stdout);
    
    // Initialize the logger
    INIT.call_once(|| {
        builder.init();
    });
    
    log::info!("Logger initialized with level: {:?}", config.log_level);
    
    Ok(())
}

fn configure_default_format(builder: &mut Builder, config: &crate::utils::UtilsConfig) {
    builder.format(move |buf, record| {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let level = record.level();
        let target = record.target();
        let args = record.args();
        
        // Colorize based on level
        let level_color = match level {
            Level::Error => "\x1b[31m", // Red
            Level::Warn => "\x1b[33m",  // Yellow
            Level::Info => "\x1b[32m",  // Green
            Level::Debug => "\x1b[36m", // Cyan
            Level::Trace => "\x1b[35m", // Magenta
        };
        let reset_color = "\x1b[0m";
        
        writeln!(
            buf,
            "{}{} {:5} [{}] {}{}: {}",
            level_color,
            timestamp,
            level,
            std::thread::current().name().unwrap_or("main"),
            target,
            reset_color,
            args
        )
    });
}

fn configure_json_format(builder: &mut Builder) {
    builder.format(|buf, record| {
        let log_entry = LogEntry::new(record);
        writeln!(buf, "{}", log_entry.to_json_string())
    });
}

fn configure_compact_format(builder: &mut Builder) {
    builder.format(|buf, record| {
        let timestamp = chrono::Local::now().format("%H:%M:%S%.3f");
        writeln!(
            buf,
            "{} {:5} {}",
            timestamp,
            record.level(),
            record.args()
        )
    });
}

fn configure_pretty_format(builder: &mut Builder) {
    builder.format(|buf, record| {
        use ansi_term::Colour;
        
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let level = match record.level() {
            Level::Error => Colour::Red.paint("ERROR"),
            Level::Warn => Colour::Yellow.paint("WARN"),
            Level::Info => Colour::Green.paint("INFO"),
            Level::Debug => Colour::Cyan.paint("DEBUG"),
            Level::Trace => Colour::Purple.paint("TRACE"),
        };
        
        writeln!(
            buf,
            "{} {} {} > {}",
            Colour::White.dimmed().paint(timestamp.to_string()),
            level,
            Colour::Blue.paint(record.target()),
            record.args()
        )
    });
}

/// Structured logging functions

/// Log with structured data
pub fn log_structured(level: Level, entry: LogEntry) {
    let message = if std::env::var("RUST_LOG_FORMAT").unwrap_or_default() == "json" {
        entry.to_json_string()
    } else {
        entry.to_plain_string()
    };
    
    log::log!(level, "{}", message);
}

/// Log info with structured data
pub fn info_structured(entry: LogEntry) {
    log_structured(Level::Info, entry);
}

/// Log warning with structured data
pub fn warn_structured(entry: LogEntry) {
    log_structured(Level::Warn, entry);
}

/// Log error with structured data
pub fn error_structured(entry: LogEntry) {
    log_structured(Level::Error, entry);
}

/// Log debug with structured data
pub fn debug_structured(entry: LogEntry) {
    log_structured(Level::Debug, entry);
}

/// Log trace with structured data
pub fn trace_structured(entry: LogEntry) {
    log_structured(Level::Trace, entry);
}

/// Request logging middleware
pub struct RequestLogger {
    enabled: bool,
    log_level: LogLevel,
    exclude_paths: Vec<String>,
    include_headers: bool,
    include_body: bool,
    max_body_size: usize,
    sensitive_headers: Vec<String>,
}

impl RequestLogger {
    pub fn new() -> Self {
        RequestLogger {
            enabled: true,
            log_level: LogLevel::Info,
            exclude_paths: vec![
                "/health".to_string(),
                "/favicon.ico".to_string(),
                "/static/".to_string(),
                "/metrics".to_string(),
            ],
            include_headers: false,
            include_body: false,
            max_body_size: 1024, // 1KB
            sensitive_headers: vec![
                "authorization".to_string(),
                "cookie".to_string(),
                "set-cookie".to_string(),
                "x-api-key".to_string(),
                "x-access-token".to_string(),
                "x-refresh-token".to_string(),
                "password".to_string(),
                "new-password".to_string(),
                "current-password".to_string(),
            ],
        }
    }
    
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }
    
    pub fn exclude_path(mut self, path: &str) -> Self {
        self.exclude_paths.push(path.to_string());
        self
    }
    
    pub fn include_headers(mut self, include: bool) -> Self {
        self.include_headers = include;
        self
    }
    
    pub fn include_body(mut self, include: bool) -> Self {
        self.include_body = include;
        self
    }
    
    pub fn with_max_body_size(mut self, size: usize) -> Self {
        self.max_body_size = size;
        self
    }
    
    pub fn add_sensitive_header(mut self, header: &str) -> Self {
        self.sensitive_headers.push(header.to_string());
        self
    }
    
    fn should_log(&self, path: &str) -> bool {
        self.enabled && !self.exclude_paths.iter().any(|exclude| path.starts_with(exclude))
    }
    
    fn mask_sensitive_headers(&self, headers: &[(String, String)]) -> Vec<(String, String)> {
        headers
            .iter()
            .map(|(name, value)| {
                if self.sensitive_headers.contains(&name.to_lowercase()) {
                    (name.clone(), "***MASKED***".to_string())
                } else {
                    (name.clone(), value.clone())
                }
            })
            .collect()
    }
    
    fn mask_sensitive_data(&self, data: &str) -> String {
        // Mask passwords and tokens in JSON data
        let patterns = vec![
            (r#""password"\s*:\s*"[^"]*""#, r#""password": "***MASKED***""#),
            (r#""new_password"\s*:\s*"[^"]*""#, r#""new_password": "***MASKED***""#),
            (r#""current_password"\s*:\s*"[^"]*""#, r#""current_password": "***MASKED***""#),
            (r#""token"\s*:\s*"[^"]*""#, r#""token": "***MASKED***""#),
            (r#""access_token"\s*:\s*"[^"]*""#, r#""access_token": "***MASKED***""#),
            (r#""refresh_token"\s*:\s*"[^"]*""#, r#""refresh_token": "***MASKED***""#),
            (r#""api_key"\s*:\s*"[^"]*""#, r#""api_key": "***MASKED***""#),
        ];
        
        let mut masked = data.to_string();
        for (pattern, replacement) in patterns {
            masked = regex::Regex::new(pattern)
                .unwrap()
                .replace_all(&masked, replacement)
                .to_string();
        }
        
        masked
    }
}

impl Default for RequestLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance monitoring
pub struct PerformanceMonitor {
    enabled: bool,
    slow_request_threshold_ms: u128,
    enable_metrics: bool,
    metrics_buffer: std::sync::Mutex<Vec<PerformanceMetric>>,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<(String, String)>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        PerformanceMonitor {
            enabled: true,
            slow_request_threshold_ms: 1000, // 1 second
            enable_metrics: true,
            metrics_buffer: std::sync::Mutex::new(Vec::new()),
        }
    }
    
    pub fn record_metric(&self, name: &str, value: f64, tags: Vec<(&str, &str)>) {
        if !self.enable_metrics {
            return;
        }
        
        let metric = PerformanceMetric {
            name: name.to_string(),
            value,
            timestamp: Utc::now(),
            tags: tags.into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        };
        
        let mut buffer = self.metrics_buffer.lock().unwrap();
        buffer.push(metric);
        
        // Keep buffer size manageable
        if buffer.len() > 1000 {
            buffer.drain(0..100);
        }
    }
    
    pub fn get_metrics(&self) -> Vec<PerformanceMetric> {
        let buffer = self.metrics_buffer.lock().unwrap();
        buffer.clone()
    }
    
    pub fn clear_metrics(&self) {
        let mut buffer = self.metrics_buffer.lock().unwrap();
        buffer.clear();
    }
}

/// Audit logging for security events
pub struct AuditLogger;

impl AuditLogger {
    /// Log a security event
    pub fn log_security_event(
        event_type: &str,
        user_id: Option<i32>,
        ip: &str,
        details: &str,
        severity: &str,
        metadata: Option<serde_json::Value>,
    ) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: match severity {
                "critical" => "ERROR",
                "high" => "ERROR",
                "medium" => "WARN",
                "low" => "INFO",
                _ => "INFO",
            }.to_string(),
            target: "audit".to_string(),
            message: format!("Security event: {} - {}", event_type, details),
            module_path: None,
            file: None,
            line: None,
            thread_id: format!("{:?}", std::thread::current().id()),
            process_id: std::process::id(),
            request_id: None,
            user_id: user_id.map(|id| id.to_string()),
            correlation_id: None,
            additional_fields: json!({
                "event_type": event_type,
                "ip_address": ip,
                "severity": severity,
                "metadata": metadata.unwrap_or(json!({})),
                "category": "security",
            }),
        };
        
        log_structured(Level::Info, entry);
    }
    
    /// Log authentication event
    pub fn log_authentication(
        user_id: i32,
        ip: &str,
        success: bool,
        method: &str,
        reason: Option<&str>,
    ) {
        let event_type = if success { "login_success" } else { "login_failed" };
        let details = format!("Authentication attempt via {}: {}", method, reason.unwrap_or(""));
        
        Self::log_security_event(
            event_type,
            Some(user_id),
            ip,
            &details,
            if success { "info" } else { "warn" },
            Some(json!({
                "method": method,
                "success": success,
                "reason": reason,
            })),
        );
    }
    
    /// Log authorization failure
    pub fn log_authorization_failure(
        user_id: i32,
        ip: &str,
        resource: &str,
        action: &str,
        required_permission: &str,
    ) {
        Self::log_security_event(
            "authorization_failure",
            Some(user_id),
            ip,
            &format!("Attempted {} on {} (required: {})", action, resource, required_permission),
            "warn",
            Some(json!({
                "resource": resource,
                "action": action,
                "required_permission": required_permission,
            })),
        );
    }
    
    /// Log sensitive operation
    pub fn log_sensitive_operation(
        user_id: i32,
        ip: &str,
        operation: &str,
        resource_type: &str,
        resource_id: Option<i32>,
        details: &str,
    ) {
        Self::log_security_event(
            "sensitive_operation",
            Some(user_id),
            ip,
            &format!("{} on {}: {}", operation, resource_type, details),
            "info",
            Some(json!({
                "operation": operation,
                "resource_type": resource_type,
                "resource_id": resource_id,
                "details": details,
            })),
        );
    }
    
    /// Log data access
    pub fn log_data_access(
        user_id: i32,
        ip: &str,
        data_type: &str,
        data_id: Option<i32>,
        access_type: &str,
        details: &str,
    ) {
        Self::log_security_event(
            "data_access",
            Some(user_id),
            ip,
            &format!("{} access to {}: {}", access_type, data_type, details),
            "info",
            Some(json!({
                "data_type": data_type,
                "data_id": data_id,
                "access_type": access_type,
                "details": details,
            })),
        );
    }
}

/// Business operation logging
pub struct BusinessLogger;

impl BusinessLogger {
    /// Log a business operation
    pub fn log_operation(
        operation: &str,
        user_id: Option<i32>,
        resource_type: &str,
        resource_id: Option<i32>,
        details: &str,
        metadata: Option<serde_json::Value>,
    ) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: "INFO".to_string(),
            target: "business".to_string(),
            message: format!("Operation: {} - {}", operation, details),
            module_path: None,
            file: None,
            line: None,
            thread_id: format!("{:?}", std::thread::current().id()),
            process_id: std::process::id(),
            request_id: None,
            user_id: user_id.map(|id| id.to_string()),
            correlation_id: None,
            additional_fields: json!({
                "operation": operation,
                "resource_type": resource_type,
                "resource_id": resource_id,
                "metadata": metadata.unwrap_or(json!({})),
                "category": "business",
            }),
        };
        
        info_structured(entry);
    }
    
    /// Log donation operation
    pub fn log_donation(
        donation_id: i32,
        user_id: i32,
        amount: f64,
        currency: &str,
        charity_id: i32,
        text_id: Option<i32>,
    ) {
        Self::log_operation(
            "donation_created",
            Some(user_id),
            "donation",
            Some(donation_id),
            &format!("Donation of {} {} to charity {}", amount, currency, charity_id),
            Some(json!({
                "amount": amount,
                "currency": currency,
                "charity_id": charity_id,
                "text_id": text_id,
            })),
        );
    }
    
    /// Log text upload
    pub fn log_text_upload(
        text_id: i32,
        user_id: i32,
        title: &str,
        version: i32,
    ) {
        Self::log_operation(
            "text_uploaded",
            Some(user_id),
            "text",
            Some(text_id),
            &format!("Text '{}' uploaded (version {})", title, version),
            Some(json!({
                "title": title,
                "version": version,
            })),
        );
    }
    
    /// Log plagiarism case
    pub fn log_plagiarism_case(
        case_id: i32,
        user_id: i32,
        text_id: i32,
        action: &str,
        details: &str,
    ) {
        Self::log_operation(
            &format!("plagiarism_case_{}", action),
            Some(user_id),
            "plagiarism_case",
            Some(case_id),
            &format!("Case for text {}: {}", text_id, details),
            Some(json!({
                "text_id": text_id,
                "action": action,
            })),
        );
    }
    
    /// Log committee action
    pub fn log_committee_action(
        committee_id: i32,
        user_id: i32,
        action: &str,
        details: &str,
        decision: Option<&str>,
    ) {
        Self::log_operation(
            &format!("committee_{}", action),
            Some(user_id),
            "committee",
            Some(committee_id),
            details,
            Some(json!({
                "action": action,
                "decision": decision,
            })),
        );
    }
}

/// Log exporter for exporting logs
pub struct LogExporter;

impl LogExporter {
    /// Export logs in specified format
    pub async fn export_logs(
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        level_filter: Option<LevelFilter>,
        format: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // In a real implementation, this would read from log files or database
        // For now, return a sample export
        
        let sample_logs = vec![
            LogEntry {
                timestamp: Utc::now(),
                level: "INFO".to_string(),
                target: "app".to_string(),
                message: "Sample log entry 1".to_string(),
                module_path: Some("module".to_string()),
                file: Some("file.rs".to_string()),
                line: Some(42),
                thread_id: "main".to_string(),
                process_id: 12345,
                request_id: Some("req-123".to_string()),
                user_id: Some("456".to_string()),
                correlation_id: Some("corr-789".to_string()),
                additional_fields: json!({}),
            },
        ];
        
        match format {
            "json" => {
                let json_logs: Vec<serde_json::Value> = sample_logs
                    .iter()
                    .map(|log| serde_json::to_value(log).unwrap())
                    .collect();
                Ok(serde_json::to_vec(&json_logs)?)
            }
            "csv" => {
                let mut csv_writer = csv::Writer::from_writer(Vec::new());
                
                for log in sample_logs {
                    csv_writer.write_record(&[
                        log.timestamp.to_rfc3339(),
                        log.level,
                        log.target,
                        log.message,
                        log.module_path.unwrap_or_default(),
                        log.file.unwrap_or_default(),
                        log.line.map(|l| l.to_string()).unwrap_or_default(),
                        log.thread_id,
                        log.process_id.to_string(),
                        log.request_id.unwrap_or_default(),
                        log.user_id.unwrap_or_default(),
                        log.correlation_id.unwrap_or_default(),
                    ])?;
                }
                
                Ok(csv_writer.into_inner()?)
            }
            "text" => {
                let text_logs: Vec<String> = sample_logs
                    .iter()
                    .map(|log| log.to_plain_string())
                    .collect();
                Ok(text_logs.join("\n").into_bytes())
            }
            _ => Err("Unsupported format".into()),
        }
    }
}

/// Log rotation utilities
pub struct LogRotator {
    log_dir: String,
    max_file_size: u64,
    max_files: u32,
    current_file: String,
}

impl LogRotator {
    pub fn new(log_dir: &str, max_file_size: u64, max_files: u32) -> Result<Self, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(log_dir)?;
        
        let current_file = format!("{}/app.log", log_dir);
        
        Ok(LogRotator {
            log_dir: log_dir.to_string(),
            max_file_size,
            max_files,
            current_file,
        })
    }
    
    pub fn rotate_if_needed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let metadata = std::fs::metadata(&self.current_file);
        
        if let Ok(metadata) = metadata {
            if metadata.len() >= self.max_file_size {
                self.perform_rotation()?;
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    fn perform_rotation(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Rename existing files
        for i in (1..self.max_files).rev() {
            let old_name = format!("{}/app.log.{}", self.log_dir, i);
            let new_name = format!("{}/app.log.{}", self.log_dir, i + 1);
            
            if std::fs::metadata(&old_name).is_ok() {
                std::fs::rename(&old_name, &new_name)?;
            }
        }
        
        // Move current file to .1
        let rotated_name = format!("{}/app.log.1", self.log_dir);
        if std::fs::metadata(&self.current_file).is_ok() {
            std::fs::rename(&self.current_file, &rotated_name)?;
        }
        
        // Create new log file
        std::fs::File::create(&self.current_file)?;
        
        log::info!("Log file rotated");
        
        Ok(())
    }
    
    pub fn cleanup_old_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        for i in (self.max_files + 1).. {
            let file_name = format!("{}/app.log.{}", self.log_dir, i);
            if std::fs::metadata(&file_name).is_ok() {
                std::fs::remove_file(file_name)?;
            } else {
                break;
            }
        }
        
        Ok(())
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_level_conversion() {
        assert_eq!(LevelFilter::from(LogLevel::Off), LevelFilter::Off);
        assert_eq!(LevelFilter::from(LogLevel::Error), LevelFilter::Error);
        assert_eq!(LevelFilter::from(LogLevel::Info), LevelFilter::Info);
        assert_eq!(LevelFilter::from(LogLevel::Debug), LevelFilter::Debug);
        
        assert_eq!(LogLevel::from("error"), LogLevel::Error);
        assert_eq!(LogLevel::from("INFO"), LogLevel::Info);
        assert_eq!(LogLevel::from("unknown"), LogLevel::Info);
    }
    
    #[test]
    fn test_log_entry_creation() {
        let record = Record::builder()
            .level(Level::Info)
            .target("test_target")
            .args(format_args!("Test message"))
            .file(Some("test.rs"))
            .line(Some(123))
            .module_path(Some("test_module"))
            .build();
        
        let entry = LogEntry::new(&record);
        
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.target, "test_target");
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.module_path, Some("test_module".to_string()));
        assert_eq!(entry.file, Some("test.rs".to_string()));
        assert_eq!(entry.line, Some(123));
    }
    
    #[test]
    fn test_log_entry_with_context() {
        let record = Record::builder()
            .level(Level::Info)
            .target("test")
            .args(format_args!("Test"))
            .build();
        
        let entry = LogEntry::new(&record)
            .with_request_id("req-123")
            .with_user_id("456")
            .with_correlation_id("corr-789")
            .with_field("custom_field", "custom_value");
        
        assert_eq!(entry.request_id, Some("req-123".to_string()));
        assert_eq!(entry.user_id, Some("456".to_string()));
        assert_eq!(entry.correlation_id, Some("corr-789".to_string()));
        assert_eq!(entry.additional_fields["custom_field"], "custom_value");
    }
    
    #[test]
    fn test_request_logger_masking() {
        let logger = RequestLogger::new();
        
        let headers = vec![
            ("Authorization".to_string(), "Bearer secret-token".to_string()),
            ("Content-Type".to_string(), "application/json".to_string()),
            ("X-Api-Key".to_string(), "secret-api-key".to_string()),
        ];
        
        let masked = logger.mask_sensitive_headers(&headers);
        
        assert_eq!(masked[0].1, "***MASKED***");
        assert_eq!(masked[1].1, "application/json");
        assert_eq!(masked[2].1, "***MASKED***");
    }
    
    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();
        
        monitor.record_metric("test_metric", 42.0, vec![("tag1", "value1")]);
        
        let metrics = monitor.get_metrics();
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].name, "test_metric");
        assert_eq!(metrics[0].value, 42.0);
        assert_eq!(metrics[0].tags[0].0, "tag1");
        assert_eq!(metrics[0].tags[0].1, "value1");
    }
}
