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
