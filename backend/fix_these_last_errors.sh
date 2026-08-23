#!/bin/bash
# fix_these_last_errors.sh

echo "Fixing the last 2 errors..."

# First, let's check what's in plagiarism_queries.rs
echo "=== Checking plagiarism_queries.rs ==="
sed -n '1,20p' src/application/queries/plagiarism_queries.rs

# Fix the import syntax
echo "Fixing plagiarism_queries.rs import..."
# Replace the line with proper syntax
sed -i 's/responses::plagiarism_response{/responses::plagiarism_response::{/' src/application/queries/plagiarism_queries.rs

# Check if it worked
echo "After fix:"
sed -n '10,12p' src/application/queries/plagiarism_queries.rs

# Now fix external/mod.rs by checking what's there
echo -e "\n=== Checking external/mod.rs ==="
sed -n '145,165p' src/infrastructure/external/mod.rs

# The issue is we have duplicate AsAny implementations. Let's just remove the problematic one
echo "Fixing external/mod.rs..."
# Backup
cp src/infrastructure/external/mod.rs /tmp/external_mod_backup.rs

# Remove lines 150-160 to get rid of the duplicate trait
sed -i '150,160d' src/infrastructure/external/mod.rs

# Or better, let's see the exact content and fix it
cat > src/infrastructure/external/mod.rs << 'EOF'
use std::any::Any;
use async_trait::async_trait;
use uuid::Uuid;

pub mod payment_gateway;

// Trait for external services
pub trait ExternalService: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

// Payment Gateway trait
#[async_trait]
pub trait PaymentGateway: ExternalService {
    async fn process_payment(
        &self,
        amount: f64,
        currency: &str,
        payment_method: &str,
        customer_id: Option<String>,
    ) -> Result<String, String>;

    async fn refund_payment(
        &self,
        payment_id: &str,
        amount: Option<f64>,
    ) -> Result<String, String>;

    async fn get_payment_status(
        &self,
        payment_id: &str,
    ) -> Result<String, String>;
}

// Email Service trait
#[async_trait]
pub trait EmailService: ExternalService {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), String>;
}

// File Storage trait
#[async_trait]
pub trait FileStorage: ExternalService {
    async fn upload_file(
        &self,
        file_name: &str,
        content: &[u8],
        content_type: &str,
    ) -> Result<String, String>;

    async fn download_file(
        &self,
        file_id: &str,
    ) -> Result<Vec<u8>, String>;

    async fn delete_file(
        &self,
        file_id: &str,
    ) -> Result<(), String>;
}

// Trait to allow downcasting - only one implementation
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

// Single implementation
impl<T: Any + Send + Sync> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
EOF

echo -e "\nRunning cargo check..."
cargo check
