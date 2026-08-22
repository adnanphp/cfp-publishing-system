pub fn log_info(message: &str) {
    println!("ℹ️  {}", message);
}

pub fn log_success(message: &str) {
    println!("✅ {}", message);
}

pub fn log_error(message: &str) {
    eprintln!("❌ {}", message);
}

pub fn log_request(method: &str, path: &str, duration_ms: u128) {
    println!("🌐 {} {} - {}ms", method, path, duration_ms);
}
