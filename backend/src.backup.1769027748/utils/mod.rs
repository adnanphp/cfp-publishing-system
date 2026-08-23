use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
// Utility modules for the CFP backend system
// 
// This module provides common utilities used throughout the application:
// - Error handling and custom error types
// - Logging configuration and utilities
// - Input validation and sanitization
// - Date/time manipulation and formatting
// - Cryptographic operations and security utilities

pub mod error;
pub mod logger;
pub mod validation;
pub mod datetime;
pub mod cryptography;

// Re-exports for convenient access
pub use error::{AppError, AppErrorType, ResultExt, ErrorResponse};
pub use logger::{setup_logger, LogLevel, RequestLogger};
pub use validation::{validate_email, validate_password, validate_orcid, ValidationResult};
pub use crate::utils::datetime::*;
pub use crate::utils::cryptography::*;

/// Common result type for the application
pub type AppResult<T> = std::result::Result<T, error::AppError>;

/// Configuration for utility modules
#[derive(Debug, Clone)]
pub struct UtilsConfig {
    pub log_level: LogLevel,
    pub date_format: String,
    pub datetime_format: String,
    pub password_hash_cost: u32,
    pub token_expiry_hours: u32,
    pub encryption_key: String,
}

impl Default for UtilsConfig {
    fn default() -> Self {
        UtilsConfig {
            log_level: LogLevel::Info,
            date_format: "%Y-%m-%d".to_string(),
            datetime_format: "%Y-%m-%d %H:%M:%S".to_string(),
            password_hash_cost: 12,
            token_expiry_hours: 24,
            encryption_key: std::env::var("ENCRYPTION_KEY").unwrap_or_else(|_| "default-secret-key-32-chars".to_string()),
        }
    }
}

/// Initialize all utility modules with configuration
pub fn initialize_utils(config: Option<UtilsConfig>) -> AppResult<()> {
    let config = config.unwrap_or_default();
    
    // Initialize logging
    logger::setup_logger(&config)?;
    
    // Validate encryption key length
    if config.encryption_key.len() < 32 {
        return Err(AppError::new(
            error::AppErrorType::Configuration,
            "Encryption key must be at least 32 characters".to_string(),
        ));
    }
    
    // Log initialization
    log::info!("Utilities initialized with config: {:?}", config);
    
    Ok(())
}

/// Generic utility functions

/// Convert a string to title case
pub fn to_title_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c.is_whitespace() || c == '-' || c == '_' {
            capitalize_next = true;
            result.push(' ');
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }
    
    result
}

/// Truncate a string with ellipsis
pub fn truncate_with_ellipsis(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        return s.to_string();
    }
    
    if max_length <= 3 {
        return ".".repeat(max_length);
    }
    
    format!("{}...", &s[..max_length - 3])
}

/// Generate a random string of specified length
pub fn generate_random_string(length: usize) -> String {
    
    use rand::{thread_rng, Rng};
    
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generate a random number within range
pub fn generate_random_number(min: u32, max: u32) -> u32 {
    use rand::{thread_rng, Rng};
    
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

/// Check if a string is empty or whitespace only
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

/// Remove all whitespace from a string
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Parse a comma-separated string into a vector
pub fn parse_csv_string(s: &str) -> Vec<String> {
    s.split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

/// Join a vector into a comma-separated string
pub fn join_csv_string(items: &[String]) -> String {
    items.join(", ")
}

/// Extract domain from email address
pub fn extract_domain_from_email(email: &str) -> Option<String> {
    email.split('@').nth(1).map(|s| s.to_string())
}

/// Calculate percentage
pub fn calculate_percentage(part: f64, total: f64) -> f64 {
    if total == 0.0 {
        return 0.0;
    }
    (part / total) * 100.0
}

/// Format bytes to human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let base: f64 = 1024.0;
    let exp = (bytes as f64).log(base).floor() as u32;
    let size = bytes as f64 / base.powi(exp as i32);
    
    format!("{:.2} {}", size, UNITS[exp as usize])
}

/// Generate a slug from a string
pub fn generate_slug(text: &str) -> String {
    use unidecode::unidecode;
    
    let slug = unidecode(text);
    slug.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
        .replace("--", "-")
}

/// Validate and normalize URL
pub fn normalize_url(url: &str) -> Option<String> {
    let url = url.trim();
    
    if url.is_empty() {
        return None;
    }
    
    // Add https:// if no scheme is present
    let normalized = if !url.starts_with("http://") && !url.starts_with("https://") {
        format!("https://{}", url)
    } else {
        url.to_string()
    };
    
    // Validate URL format
    match url::Url::parse(&normalized) {
        Ok(parsed_url) => {
            // Ensure it's a valid web URL
            if parsed_url.scheme() == "http" || parsed_url.scheme() == "https" {
                Some(parsed_url.to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Safe integer parsing with default value
pub fn safe_parse_int(s: &str, default: i32) -> i32 {
    s.parse().unwrap_or(default)
}

/// Safe float parsing with default value
pub fn safe_parse_float(s: &str, default: f64) -> f64 {
    s.parse().unwrap_or(default)
}

/// Check if a string contains only ASCII characters
pub fn is_ascii_only(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii())
}

/// Remove HTML tags from a string
pub fn strip_html_tags(s: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ => {
                if !in_tag {
                    result.push(c);
                }
            }
        }
    }
    
    result
}

/// Count words in a string
pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Count characters in a string (excluding whitespace)
pub fn count_characters(s: &str) -> usize {
    s.chars().filter(|c| !c.is_whitespace()).count()
}

/// Calculate reading time in minutes
pub fn calculate_reading_time(word_count: usize, words_per_minute: usize) -> usize {
    let minutes = word_count / words_per_minute;
    minutes.max(1)
}

/// Mask sensitive information (like email addresses)
pub fn mask_sensitive_info(s: &str) -> String {
    if s.contains('@') {
        // Email address
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() == 2 {
            let username = parts[0];
            let domain = parts[1];
            
            if username.len() <= 2 {
                format!("*@{}", domain)
            } else {
                let masked = format!("{}***{}", &username[0..1], &username[username.len()-1..]);
                format!("{}@{}", masked, domain)
            }
        } else {
            "***@***".to_string()
        }
    } else if s.len() > 4 {
        // General string
        format!("{}***{}", &s[0..2], &s[s.len()-2..])
    } else {
        "***".to_string()
    }
}

/// Generate a unique identifier
pub fn generate_unique_id() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

/// Parse a string into a boolean
pub fn parse_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "y" | "on" => Some(true),
        "false" | "0" | "no" | "n" | "off" => Some(false),
        _ => None,
    }
}

/// Utility trait for converting between types
pub trait Convert {
    fn to_json(&self) -> serde_json::Value;
    fn to_yaml(&self) -> Result<String, serde_yaml::Error>;
}

/// Implementation for common types
impl<T: serde::Serialize> Convert for T {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
    
    fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

/// Progress tracker for long-running operations
pub struct ProgressTracker {
    total: u64,
    current: u64,
    started_at: std::time::Instant,
}

impl ProgressTracker {
    pub fn new(total: u64) -> Self {
        ProgressTracker {
            total,
            current: 0,
            started_at: std::time::Instant::now(),
        }
    }
    
    pub fn increment(&mut self) {
        self.current += 1;
    }
    
    pub fn set_progress(&mut self, current: u64) {
        self.current = current;
    }
    
    pub fn get_progress(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.current as f64 / self.total as f64) * 100.0
    }
    
    pub fn get_elapsed_time(&self) -> std::time::Duration {
        self.started_at.elapsed()
    }
    
    pub fn get_estimated_time_remaining(&self) -> Option<std::time::Duration> {
        if self.current == 0 {
            return None;
        }
        
        let elapsed = self.get_elapsed_time();
        let time_per_item = elapsed / self.current;
        let remaining_items = self.total - self.current;
        
        Some(time_per_item * remaining_items)
    }
    
    pub fn get_progress_string(&self) -> String {
        let progress = self.get_progress();
        let elapsed = self.get_elapsed_time();
        
        if let Some(eta) = self.get_estimated_time_remaining() {
            format!("{:.1}% ({}s elapsed, {}s remaining)", 
                progress, 
                elapsed.as_secs(),
                eta.as_secs())
        } else {
            format!("{:.1}% ({}s elapsed)", progress, elapsed.as_secs())
        }
    }
}

/// Retry mechanism for operations that might fail
pub async fn retry_operation<F, T, E>(
    operation: F,
    max_retries: u32,
    initial_delay_ms: u64,
) -> Result<T, E>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>>>>,
    E: std::fmt::Display + std::fmt::Debug,
{
    let mut retries = 0;
    let mut delay = initial_delay_ms;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                retries += 1;
                
                if retries >= max_retries {
                    log::error!("Operation failed after {} retries: {}", retries, err);
                    return Err(err);
                }
                
                log::warn!("Operation failed (attempt {}/{}), retrying in {}ms: {}", 
                    retries, max_retries, delay, err);
                
                // Exponential backoff with jitter
                let jitter = rand::random::<u64>() % 100;
                tokio::time::sleep(tokio::time::Duration::from_millis(delay + jitter)).await;
                
                delay = delay * 2; // Exponential backoff
            }
        }
    }
}

/// Cache wrapper with TTL support
pub struct Cache<K, V> {
    store: std::collections::HashMap<K, (V, std::time::Instant)>,
    ttl: std::time::Duration,
}

impl<K, V> Cache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    pub fn new(ttl: std::time::Duration) -> Self {
        Cache {
            store: std::collections::HashMap::new(),
            ttl,
        }
    }
    
    pub fn insert(&mut self, key: K, value: V) {
        self.store.insert(key, (value, std::time::Instant::now()));
        self.cleanup();
    }
    
    pub fn get(&mut self, key: &K) -> Option<V> {
        self.cleanup();
        
        self.store.get(key).and_then(|(value, timestamp)| {
            if timestamp.elapsed() < self.ttl {
                Some(value.clone())
            } else {
                None
            }
        })
    }
    
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key).map(|(value, _)| value)
    }
    
    pub fn clear(&mut self) {
        self.store.clear();
    }
    
    pub fn size(&self) -> usize {
        self.store.len()
    }
    
    fn cleanup(&mut self) {
        let now = std::time::Instant::now();
        self.store.retain(|_, (_, timestamp)| {
            now.duration_since(*timestamp) < self.ttl
        });
    }
}

/// Async cache with TTL support
pub struct AsyncCache<K, V> {
    cache: tokio::sync::RwLock<Cache<K, V>>,
}

impl<K, V> AsyncCache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    pub fn new(ttl: std::time::Duration) -> Self {
        AsyncCache {
            cache: tokio::sync::RwLock::new(Cache::new(ttl)),
        }
    }
    
    pub async fn insert(&self, key: K, value: V) {
        let mut cache = self.cache.write().await;
        cache.insert(key, value);
    }
    
    pub async fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.write().await;
        cache.get(key)
    }
    
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.write().await;
        cache.remove(key)
    }
    
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
    
    pub async fn size(&self) -> usize {
        let cache = self.cache.read().await;
        cache.size()
    }
}

// Unit tests for utility functions
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("hello-world"), "Hello World");
        assert_eq!(to_title_case("hello_world"), "Hello World");
        assert_eq!(to_title_case("HELLO WORLD"), "Hello World");
    }
    
    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello", 5), "hello");
        assert_eq!(truncate_with_ellipsis("hello", 3), "...");
        assert_eq!(truncate_with_ellipsis("hello world", 8), "hello...");
    }
    
    #[test]
    fn test_is_blank() {
        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(is_blank("\t\n"));
        assert!(!is_blank("hello"));
        assert!(!is_blank(" hello "));
    }
    
    #[test]
    fn test_parse_csv_string() {
        assert_eq!(parse_csv_string("a,b,c"), vec!["a", "b", "c"]);
        assert_eq!(parse_csv_string("a, b, c"), vec!["a", "b", "c"]);
        assert_eq!(parse_csv_string("a,,c"), vec!["a", "c"]);
        assert_eq!(parse_csv_string(""), Vec::<String>::new());
    }
    
    #[test]
    fn test_extract_domain_from_email() {
        assert_eq!(extract_domain_from_email("test@example.com"), Some("example.com".to_string()));
        assert_eq!(extract_domain_from_email("invalid-email"), None);
    }
    
    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(50.0, 100.0), 50.0);
        assert_eq!(calculate_percentage(0.0, 100.0), 0.0);
        assert_eq!(calculate_percentage(100.0, 0.0), 0.0);
    }
    
    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hello World!"), "hello-world");
        assert_eq!(generate_slug("Test & Test"), "test-test");
        assert_eq!(generate_slug("  Test  "), "test");
    }
    
    #[test]
    fn test_normalize_url() {
        assert_eq!(normalize_url("example.com"), Some("https://example.com/".to_string()));
        assert_eq!(normalize_url("http://example.com"), Some("http://example.com/".to_string()));
        assert_eq!(normalize_url(""), None);
    }
    
    #[test]
    fn test_mask_sensitive_info() {
        assert_eq!(mask_sensitive_info("test@example.com"), "t***t@example.com");
        assert_eq!(mask_sensitive_info("ab@example.com"), "*@example.com");
        assert_eq!(mask_sensitive_info("password"), "pa***rd");
        assert_eq!(mask_sensitive_info("ab"), "***");
    }
}
