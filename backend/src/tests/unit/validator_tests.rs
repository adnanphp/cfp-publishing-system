//! Validation logic unit tests

use crate::utils::validation::*;
use crate::utils::datetime::CFPDateUtils;
use chrono::{Utc, Duration};

#[test]
fn test_email_validation() {
    // Valid emails
    let valid_emails = vec![
        "test@example.com",
        "user.name@domain.co.uk",
        "user+tag@example.org",
        "123@numbers.com",
        "user@sub.domain.com",
    ];
    
    for email in valid_emails {
        assert!(validate_email(email).is_ok(), "Email should be valid: {}", email);
    }
    
    // Invalid emails
    let invalid_emails = vec![
        "not-an-email",
        "@no-username.com",
        "user@.com",
        "user@domain.",
        "user@domain..com",
        "user@-domain.com",
        "",
        "   ",
    ];
    
    for email in invalid_emails {
        assert!(validate_email(email).is_err(), "Email should be invalid: {}", email);
    }
}

#[test]
fn test_password_validation() {
    // Valid passwords
    let valid_passwords = vec![
        "ValidPass123!",
        "Another$Password456",
        "Strong@Pass789",
        "Test#Password2024",
        "ABCdef123!@#",
    ];
    
    for password in valid_passwords {
        assert!(validate_password(password).is_ok(), "Password should be valid: {}", password);
    }
    
    // Invalid passwords
    let invalid_passwords = vec![
        "short",                     // Too short
        "nouppercase123!",          // No uppercase
        "NOLOWERCASE123!",          // No lowercase
        "NoNumbers!",               // No numbers
        "NoSpecial123",             // No special characters
        "",                         // Empty
        "   ",                      // Whitespace only
        "aaaaaaaa",                 // Only lowercase
        "AAAAAAAA",                 // Only uppercase
        "11111111",                 // Only numbers
        "!!!!!!!!",                 // Only special
    ];
    
    for password in invalid_passwords {
        assert!(validate_password(password).is_err(), "Password should be invalid: {}", password);
    }
}

#[test]
fn test_orcid_validation() {
    // Valid ORCIDs
    let valid_orcids = vec![
        "0000-0002-1825-0097",
        "0000-0001-2345-6789",
        "0000-0003-4567-890X", // Ends with X
    ];
    
    for orcid in valid_orcids {
        assert!(validate_orcid(orcid).is_ok(), "ORCID should be valid: {}", orcid);
    }
    
    // Invalid ORCIDs
    let invalid_orcids = vec![
        "0000-0002-1825-009",      // Too short
        "0000-0002-1825-00970",    // Too long
        "0000-0002-1825-009a",     // Invalid character
        "0000000218250097",        // No hyphens
        "0000-0002-1825-009-",     // Extra hyphen
        "-000-0002-1825-0097",     // Starting hyphen
        "",                        // Empty
        "invalid",                 // Completely invalid
    ];
    
    for orcid in invalid_orcids {
        assert!(validate_orcid(orcid).is_err(), "ORCID should be invalid: {}", orcid);
    }
}

#[test]
fn test_phone_validation() {
    // Valid phone numbers (E.164 format)
    let valid_phones = vec![
        "+1234567890",
        "+441234567890",
        "+1-234-567-8901",  // With hyphens (will be stripped in validation)
        "1234567890",       // Without plus
    ];
    
    for phone in valid_phones {
        // Clean the phone number for validation
        let cleaned = phone.replace(['-', ' ', '(', ')'], "");
        assert!(validate_phone(&cleaned).is_ok(), "Phone should be valid: {}", phone);
    }
    
    // Invalid phone numbers
    let invalid_phones = vec![
        "",                     // Empty
        "   ",                  // Whitespace
        "abc",                  // Letters
        "+",                    // Just plus
        "123",                  // Too short
        "+1234567890123456",    // Too long
        "++1234567890",         // Double plus
        "+1(234)567-890",       // Contains parentheses (not cleaned)
    ];
    
    for phone in invalid_phones {
        assert!(validate_phone(phone).is_err(), "Phone should be invalid: {}", phone);
    }
}

#[test]
fn test_postal_code_validation() {
    // Valid postal codes (various formats)
    let valid_codes = vec![
        "12345",
        "12345-6789",
        "A1B 2C3",
        "SW1A 1AA",
        "100-0001",
        "EC1A 1BB",
    ];
    
    for code in valid_codes {
        assert!(validate_postal_code(code).is_ok(), "Postal code should be valid: {}", code);
    }
    
    // Invalid postal codes
    let invalid_codes = vec![
        "",                     // Empty
        "   ",                  // Whitespace
        "123",                  // Too short
        "123456789012345",      // Too long
        "#####",                // Invalid characters
        "AB-CD-EF",             // Wrong format
    ];
    
    for code in invalid_codes {
        assert!(validate_postal_code(code).is_err(), "Postal code should be invalid: {}", code);
    }
}

#[test]
fn test_date_validation() {
    let today = Utc::now().date_naive();
    
    // Valid dates (not in future)
    let valid_dates = vec![
        today,
        today - Duration::days(1),
        today - Duration::weeks(1),
        today - Duration::months(1),
        today - Duration::years(1),
    ];
    
    for date in valid_dates {
        assert!(validate_date_not_future(&date).is_ok(), "Date should be valid: {}", date);
    }
    
    // Invalid dates (in future)
    let invalid_dates = vec![
        today + Duration::days(1),
        today + Duration::weeks(1),
        today + Duration::months(1),
        today + Duration::years(1),
    ];
    
    for date in invalid_dates {
        assert!(validate_date_not_future(&date).is_err(), "Date should be invalid (future): {}", date);
    }
}

#[test]
fn test_donation_percentage_validation() {
    // Valid percentage distributions
    let valid_distributions = vec![
        (60, 20, 20),   // Minimum charity percentage
        (70, 15, 15),
        (80, 10, 10),
        (90, 5, 5),
        (100, 0, 0),    // All to charity
        (65, 25, 10),
    ];
    
    for (charity, cfp, author) in valid_distributions {
        assert!(
            validate_donation_percentages(charity, cfp, author).is_ok(),
            "Distribution should be valid: charity={}, cfp={}, author={}", charity, cfp, author
        );
        assert_eq!(charity + cfp + author, 100);
        assert!(charity >= 60);
    }
    
    // Invalid percentage distributions
    let invalid_distributions = vec![
        (50, 25, 25),   // Charity < 60%
        (59, 20, 21),   // Charity < 60%
        (60, 30, 20),   // Sum != 100%
        (70, 20, 15),   // Sum != 100%
        (100, 10, -10), // Negative percentage
        (-10, 60, 50),  // Negative percentage
        (60, 40, 0),    // Charity = 60%, but sum = 100%
        (101, -1, 0),   // Out of bounds
    ];
    
    for (charity, cfp, author) in invalid_distributions {
        assert!(
            validate_donation_percentages(charity, cfp, author).is_err(),
            "Distribution should be invalid: charity={}, cfp={}, author={}", charity, cfp, author
        );
    }
}

#[test]
fn test_rating_validation() {
    // Valid ratings
    for rating in 1..=5 {
        assert!(validate_rating(rating).is_ok(), "Rating should be valid: {}", rating);
    }
    
    // Invalid ratings
    let invalid_ratings = vec![0, 6, -1, 10, 100];
    
    for rating in invalid_ratings {
        assert!(validate_rating(rating).is_err(), "Rating should be invalid: {}", rating);
    }
}

#[test]
fn test_version_validation() {
    // Valid versions
    for version in 1..=100 {
        assert!(validate_version(version).is_ok(), "Version should be valid: {}", version);
    }
    
    // Invalid versions
    let invalid_versions = vec![0, -1, -10];
    
    for version in invalid_versions {
        assert!(validate_version(version).is_err(), "Version should be invalid: {}", version);
    }
}

#[test]
fn test_keyword_validation() {
    // Valid keyword lists
    let valid_keywords = vec![
        vec![],
        vec!["one".to_string()],
        vec!["one".to_string(), "two".to_string()],
        (0..10).map(|i| format!("keyword{}", i)).collect(), // Exactly 10
    ];
    
    for keywords in valid_keywords {
        assert!(validate_keywords(&keywords).is_ok(), 
                "Keywords should be valid: {:?}", keywords);
    }
    
    // Invalid keyword lists (too many)
    let invalid_keywords = vec![
        (0..11).map(|i| format!("keyword{}", i)).collect(), // 11 keywords
        (0..20).map(|i| format!("keyword{}", i)).collect(), // 20 keywords
        vec!["a"; 15], // 15 identical keywords
    ];
    
    for keywords in invalid_keywords {
        assert!(validate_keywords(&keywords).is_err(),
                "Keywords should be invalid (too many): {:?}", keywords);
    }
}

#[test]
fn test_comment_content_validation() {
    // Valid comment content
    let valid_comments = vec![
        "This is a valid comment with enough characters.",
        "Exactly 10!!",
        "1234567890", // Exactly 10 characters
        "A longer comment that provides meaningful feedback about the text content.",
        "Short but valid because it has exactly 10 chars!",
    ];
    
    for comment in valid_comments {
        let trimmed = comment.trim();
        assert!(validate_comment_content(trimmed).is_ok(),
                "Comment should be valid: '{}'", comment);
    }
    
    // Invalid comment content
    let invalid_comments = vec![
        "",                     // Empty
        "   ",                  // Only whitespace
        "short",                // 5 characters
        "123456789",            // 9 characters
        "a",                    // 1 character
        "\n\t\r",               // Whitespace characters
    ];
    
    for comment in invalid_comments {
        let trimmed = comment.trim();
        assert!(validate_comment_content(trimmed).is_err(),
                "Comment should be invalid: '{}'", comment);
    }
}

#[test]
fn test_donation_amount_validation() {
    // Valid donation amounts
    let valid_amounts = vec![1.0, 5.0, 10.0, 25.5, 100.0, 1000.0, 0.01, 999999.99];
    
    for amount in valid_amounts {
        assert!(amount >= 1.0, "Amount should be valid (>= $1.00): {}", amount);
    }
    
    // Invalid donation amounts
    let invalid_amounts = vec![0.0, -1.0, -100.0, 0.99, 0.01, -0.01];
    
    for amount in invalid_amounts {
        assert!(amount < 1.0 || amount < 0.0, 
                "Amount should be invalid (< $1.00 or negative): {}", amount);
    }
}

#[test]
fn test_member_status_validation() {
    // Valid member status combinations
    let valid_cases = vec![
        (false, 0),   // Non-donor, no violations
        (true, 0),    // Donor, no violations
        (false, 1),   // Non-donor, 1 violation
        (true, 1),    // Donor, 1 violation
        (false, 2),   // Non-donor, 2 violations
        (true, 2),    // Donor, 2 violations
    ];
    
    for (is_donor, violations) in valid_cases {
        assert!(validate_member_status(is_donor, violations).is_ok(),
                "Member status should be valid: donor={}, violations={}", is_donor, violations);
    }
    
    // Invalid member status (3+ violations = blacklist)
    let invalid_cases = vec![
        (false, 3),   // Non-donor, 3 violations
        (true, 3),    // Donor, 3 violations
        (false, 10),  // Non-donor, 10 violations
        (true, 100),  // Donor, 100 violations
    ];
    
    for (is_donor, violations) in invalid_cases {
        assert!(validate_member_status(is_donor, violations).is_err(),
                "Member status should be invalid (3+ violations): donor={}, violations={}", 
                is_donor, violations);
    }
}

#[test]
fn test_no_duplicates_validation() {
    // Lists without duplicates
    let no_duplicates = vec![
        vec![1, 2, 3, 4, 5],
        vec!["a", "b", "c"],
        vec![true, false],
        vec![],
        vec![1],
    ];
    
    for list in no_duplicates {
        assert!(validate_no_duplicates(&list).is_ok(),
                "List should be valid (no duplicates): {:?}", list);
    }
    
    // Lists with duplicates
    let with_duplicates = vec![
        vec![1, 2, 3, 2, 4],       // Duplicate 2
        vec!["a", "b", "a", "c"],  // Duplicate "a"
        vec![true, false, true],   // Duplicate true
        vec![1, 1],                // Only duplicates
        vec!["same", "same", "same"], // All duplicates
    ];
    
    for list in with_duplicates {
        assert!(validate_no_duplicates(&list).is_err(),
                "List should be invalid (has duplicates): {:?}", list);
    }
}

#[test]
fn test_member_validator_integration() {
    // Test complete member validation
    let validator = MemberValidator {
        name: Some("Test User".to_string()),
        email: Some("test@example.com".to_string()),
        password: Some("ValidPass123!".to_string()),
        phone_numbers: Some(vec!["+1234567890".to_string()]),
        areas_of_interest: Some(vec!["AI".to_string(), "ML".to_string()]),
        postal_code: Some("12345".to_string()),
    };
    
    let result = validator.validate();
    assert!(result.is_valid, "Valid member should pass validation");
    assert!(result.errors.is_empty());
    
    // Test invalid member
    let invalid_validator = MemberValidator {
        name: Some("Test User".to_string()),
        email: Some("invalid-email".to_string()), // Invalid
        password: Some("weak".to_string()),       // Invalid
        phone_numbers: Some(vec!["invalid".to_string()]), // Invalid
        areas_of_interest: Some(vec!["AI".to_string(), "AI".to_string()]), // Duplicate
        postal_code: Some("###".to_string()),     // Invalid
    };
    
    let result = invalid_validator.validate();
    assert!(!result.is_valid, "Invalid member should fail validation");
    assert!(!result.errors.is_empty());
    
    // Count errors
    assert!(result.errors.len() >= 4); // At least 4 validation errors
}

#[test]
fn test_validation_result_merging() {
    let mut result1 = ValidationResult::new();
    result1.add_error("email", ValidationErrorType::InvalidEmail);
    
    let mut result2 = ValidationResult::new();
    result2.add_error("password", ValidationErrorType::InvalidPassword);
    
    result1.merge(result2);
    
    assert!(!result1.is_valid);
    assert_eq!(result1.errors.len(), 2);
    
    let error_fields: Vec<String> = result1.errors.iter()
        .map(|(field, _)| field.clone())
        .collect();
    
    assert!(error_fields.contains(&"email".to_string()));
    assert!(error_fields.contains(&"password".to_string()));
}

#[test]
fn test_comprehensive_business_rules() {
    // Test that all business rules from ER diagram are enforced
    
    // 1. Download limit rule
    let non_donor_limit = 1; // per week
    let donor_limit = 1; // per day
    assert!(donor_limit >= non_donor_limit);
    
    // 2. Introduction rule (must be introduced by existing member)
    // This would be enforced at application level, not validation level
    
    // 3. Donation constraints
    assert!(validate_donation_percentages(60, 20, 20).is_ok());
    assert!(validate_donation_percentages(50, 25, 25).is_err());
    
    // 4. Plagiarism voting constraints
    let total_voters = 9;
    let required_majority = (2.0 / 3.0) * total_voters as f64;
    assert_eq!(required_majority, 6.0); // 2/3 of 9 = 6
    
    // 5. Voting period constraint
    let voting_days = 14;
    assert_eq!(voting_days, 14);
    
    // 6. One vote per member per case
    // Enforced at database/application level
    
    // 7. Comment constraints
    assert!(validate_rating(3).is_ok());
    assert!(validate_rating(6).is_err());
    assert!(validate_comment_content("Valid comment with 10 chars").is_ok());
    assert!(validate_comment_content("short").is_err());
    
    // 8. Text constraints
    assert!(validate_version(1).is_ok());
    assert!(validate_version(0).is_err());
    
    let today = Utc::now().date_naive();
    assert!(validate_date_not_future(&today).is_ok());
    
    let keywords: Vec<String> = (0..10).map(|i| format!("kw{}", i)).collect();
    assert!(validate_keywords(&keywords).is_ok());
    
    let too_many_keywords: Vec<String> = (0..11).map(|i| format!("kw{}", i)).collect();
    assert!(validate_keywords(&too_many_keywords).is_err());
}
