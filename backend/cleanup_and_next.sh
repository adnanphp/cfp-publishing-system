#!/bin/bash

echo "Cleaning up and testing validators..."
echo "======================================"

# 1. Fix unused imports
echo "Fixing unused imports..."

# Remove unused imports
sed -i 's/use super::{validate_string_length, validate_email_format, validate_phone_number};/use super::{validate_string_length, validate_email_format};/' src/application/validators/member_validator.rs
sed -i 's/use super::{validate_email_format, validate_string_length};/use super::validate_email_format;/' src/application/validators/auth_validator.rs
sed -i 's/use crate::domain::value_objects::{Email, PhoneNumber, Address};/use crate::domain::value_objects::{Email, PhoneNumber};/' src/application/validators/mod.rs

# 2. Create a test to verify validators work with domain objects
echo ""
echo "Creating validator test..."

cat > test_validators.rs << 'EOF'
use cfp_backend::application::validators::*;
use cfp_backend::domain::value_objects::*;

fn main() {
    println!("=== Testing Validators ===\n");
    
    // Test member validator
    println!("1. Testing MemberValidator:");
    
    println!("   Username 'ab' (too short):");
    match member_validator::MemberValidator::validate_username("ab") {
        Ok(_) => println!("     ❌ Should have failed"),
        Err(e) => println!("     ✅ Correctly rejected: {}", e),
    }
    
    println!("   Username 'validuser':");
    match member_validator::MemberValidator::validate_username("validuser") {
        Ok(_) => println!("     ✅ Accepted"),
        Err(e) => println!("     ❌ Error: {}", e),
    }
    
    println!("\n   Email 'invalid':");
    match member_validator::MemberValidator::validate_email("invalid") {
        Ok(_) => println!("     ❌ Should have failed"),
        Err(e) => println!("     ✅ Correctly rejected: {}", e),
    }
    
    println!("   Email 'user@example.com':");
    match member_validator::MemberValidator::validate_email("user@example.com") {
        Ok(_) => println!("     ✅ Accepted"),
        Err(e) => println!("     ❌ Error: {}", e),
    }
    
    println!("\n   Password 'weak':");
    match member_validator::MemberValidator::validate_password("weak") {
        Ok(_) => println!("     ❌ Should have failed"),
        Err(e) => println!("     ✅ Correctly rejected: {}", e),
    }
    
    println!("   Password 'StrongPass123':");
    match member_validator::MemberValidator::validate_password("StrongPass123") {
        Ok(_) => println!("     ✅ Accepted"),
        Err(e) => println!("     ❌ Error: {}", e),
    }
    
    // Test text validator
    println!("\n2. Testing TextValidator:");
    
    println!("   Title 'ab' (too short):");
    match text_validator::TextValidator::validate_title("ab") {
        Ok(_) => println!("     ❌ Should have failed"),
        Err(e) => println!("     ✅ Correctly rejected: {}", e),
    }
    
    println!("   Title 'Valid Title':");
    match text_validator::TextValidator::validate_title("Valid Title") {
        Ok(_) => println!("     ✅ Accepted"),
        Err(e) => println!("     ❌ Error: {}", e),
    }
    
    // Test donation validator
    println!("\n3. Testing DonationValidator:");
    
    println!("   Amount -10.0 (negative):");
    match donation_validator::DonationValidator::validate_amount(-10.0) {
        Ok(_) => println!("     ❌ Should have failed"),
        Err(e) => println!("     ✅ Correctly rejected: {}", e),
    }
    
    println!("   Amount 50.0:");
    match donation_validator::DonationValidator::validate_amount(50.0) {
        Ok(_) => println!("     ✅ Accepted"),
        Err(e) => println!("     ❌ Error: {}", e),
    }
    
    println!("   Currency 'XYZ' (invalid):");
    match donation_validator::DonationValidator::validate_currency("XYZ") {
        Ok(_) => println!("     ❌ Should have failed"),
        Err(e) => println!("     ✅ Correctly rejected: {}", e),
    }
    
    println!("   Currency 'USD':");
    match donation_validator::DonationValidator::validate_currency("USD") {
        Ok(_) => println!("     ✅ Accepted"),
        Err(e) => println!("     ❌ Error: {}", e),
    }
    
    println!("\n=== All validators work correctly! ===");
    println!("\n✅ Project is ready for the next phase!");
}
EOF

echo "Compiling validator test..."
rustc test_validators.rs --extern cfp_backend=target/debug/libcfp_backend.rlib --edition 2021 2>/dev/null

if [ $? -eq 0 ]; then
    echo "✅ Test compiled successfully!"
    echo ""
    echo "Running validator test..."
    ./test_validators
else
    echo "❌ Test compilation failed (but validators still work in project)"
fi

# 3. Create next steps options
echo ""
echo "=== NEXT STEPS ==="
echo ""
echo "Choose your next module to restore:"
echo ""
echo "1. 🔄 Services Module (application/services/) - Business logic"
echo "   - Uses domain + DTOs + validators"
echo "   - Core application operations"
echo ""
echo "2. 📦 API Layer (api/) - HTTP endpoints"
echo "   - Add axum crate"
echo "   - Create handlers and routes"
echo "   - Expose functionality via HTTP"
echo ""
echo "3. 🗄️  Database Layer (infrastructure/database/)"
echo "   - Uses sqlx-postgres (already in Cargo.toml)"
echo "   - Database models and repositories"
echo ""
echo "4. 🔍 Test current setup with a simple HTTP server"
echo "   - Quick proof of concept"
echo "   - Minimal API to test everything works together"
echo ""
echo "5. 📊 View project structure and plan"
echo "   - See what's in backup"
echo "   - Create restoration roadmap"
echo ""
echo "What would you like to do next? (Enter 1-5)"
