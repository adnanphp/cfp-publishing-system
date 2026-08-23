#!/bin/bash
# fix_final_errors.sh

echo "Fixing the final 2 errors..."

# Error 1: Fix plagiarism_queries.rs import syntax
echo "Fixing plagiarism_queries.rs..."
sed -i 's/responses::plagiarism_response{/responses::plagiarism_response::{/' src/application/queries/plagiarism_queries.rs

# Error 2: Fix external/mod.rs syntax error
echo "Fixing external/mod.rs..."
# Let's see what's around line 156
echo "Lines 150-160 of external/mod.rs:"
sed -n '150,160p' src/infrastructure/external/mod.rs

# The error suggests there's an extra closing brace. Let's fix it.
# Create a backup
cp src/infrastructure/external/mod.rs /tmp/external_backup.rs

# Remove the problematic lines (likely the blanket impl we tried to remove earlier)
sed -i '155,160d' src/infrastructure/external/mod.rs

# Or better, let's create a clean version of the file
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

// Trait to allow downcasting
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Send + Sync> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
EOF

echo "Fixes applied! Running cargo check..."
cargo check
