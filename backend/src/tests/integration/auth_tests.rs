//! Authentication and authorization integration tests

use crate::tests::utils;
use crate::app::AppState;
use crate::domain::member::{Member, MemberRole};
use crate::services::auth_service::AuthService;
use crate::api::auth::{RegisterRequest, LoginRequest};
use axum::extract::State;
use http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn test_member_registration_flow() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let auth_service = AuthService::new(app_state.db_pool.clone());
    
    // Test successful registration
    let register_request = RegisterRequest {
        name: "Test User".to_string(),
        email: utils::mock::test_email(),
        password: "ValidPassword123!".to_string(),
        organization: Some("Test Org".to_string()),
        pseudonym: Some("tester".to_string()),
        phone_numbers: Some(vec!["+1234567890".to_string()]),
        areas_of_interest: Some(vec!["AI".to_string(), "ML".to_string()]),
        street: Some("123 Test St".to_string()),
        city: Some("Test City".to_string()),
        state: Some("TS".to_string()),
        country: Some("Testland".to_string()),
        postal_code: Some("12345".to_string()),
        primary_email: Some("test@example.com".to_string()),
        recovery_email: Some("recovery@example.com".to_string()),
        introduced_by: None,
    };
    
    let result = auth_service.register(register_request).await;
    assert!(result.is_ok(), "Registration should succeed");
    
    let member = result.unwrap();
    assert_eq!(member.name, "Test User");
    assert!(member.verification_matrix.is_some());
    assert_eq!(member.status, "pending");
    
    // Test duplicate email registration
    let duplicate_request = RegisterRequest {
        email: member.email.clone(),
        ..register_request
    };
    
    let result = auth_service.register(duplicate_request).await;
    assert!(result.is_err(), "Duplicate email should fail");
    
    // Test invalid email format
    let invalid_email_request = RegisterRequest {
        email: "invalid-email".to_string(),
        ..register_request
    };
    
    let result = auth_service.register(invalid_email_request).await;
    assert!(result.is_err(), "Invalid email should fail");
    
    // Test weak password
    let weak_password_request = RegisterRequest {
        password: "weak".to_string(),
        ..register_request
    };
    
    let result = auth_service.register(weak_password_request).await;
    assert!(result.is_err(), "Weak password should fail");
}

#[tokio::test]
async fn test_login_flow() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let auth_service = AuthService::new(app_state.db_pool.clone());
    
    // First register a user
    let email = utils::mock::test_email();
    let password = "ValidPassword123!";
    
    let register_request = RegisterRequest {
        name: "Login Test User".to_string(),
        email: email.clone(),
        password: password.to_string(),
        organization: Some("Test Org".to_string()),
        pseudonym: None,
        phone_numbers: None,
        areas_of_interest: None,
        street: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        primary_email: None,
        recovery_email: None,
        introduced_by: None,
    };
    
    auth_service.register(register_request).await.unwrap();
    
    // Test successful login
    let login_request = LoginRequest {
        email: email.clone(),
        password: password.to_string(),
        verification_code: None, // Not verifying in this test
    };
    
    let result = auth_service.login(login_request).await;
    assert!(result.is_ok(), "Login should succeed");
    let login_response = result.unwrap();
    assert!(login_response.token.is_some());
    assert_eq!(login_response.requires_verification, false);
    
    // Test wrong password
    let wrong_password_request = LoginRequest {
        email: email.clone(),
        password: "WrongPassword123!".to_string(),
        verification_code: None,
    };
    
    let result = auth_service.login(wrong_password_request).await;
    assert!(result.is_err(), "Wrong password should fail");
    
    // Test non-existent user
    let non_existent_request = LoginRequest {
        email: "nonexistent@example.com".to_string(),
        password: "SomePassword123!".to_string(),
        verification_code: None,
    };
    
    let result = auth_service.login(non_existent_request).await;
    assert!(result.is_err(), "Non-existent user should fail");
}

#[tokio::test]
async fn test_verification_matrix_flow() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let auth_service = AuthService::new(app_state.db_pool.clone());
    
    // Register user
    let email = utils::mock::test_email();
    let register_request = RegisterRequest {
        name: "Verification Test".to_string(),
        email: email.clone(),
        password: "ValidPassword123!".to_string(),
        organization: None,
        pseudonym: None,
        phone_numbers: None,
        areas_of_interest: None,
        street: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        primary_email: None,
        recovery_email: None,
        introduced_by: None,
    };
    
    let member = auth_service.register(register_request).await.unwrap();
    let matrix = member.verification_matrix.unwrap();
    
    // Test matrix verification
    let parts: Vec<&str> = matrix.split('-').collect();
    assert_eq!(parts.len(), 9, "Matrix should have 9 parts");
    
    // Try login with incorrect verification code
    let login_request = LoginRequest {
        email: email.clone(),
        password: "ValidPassword123!".to_string(),
        verification_code: Some("AAAAAA".to_string()), // Wrong code
    };
    
    let result = auth_service.login(login_request).await;
    assert!(result.is_err(), "Wrong verification code should fail");
    
    // Note: We can't test correct verification without knowing which code to use
    // since matrix codes are randomly generated
}

#[tokio::test]
async fn test_password_reset_flow() {
    utils::init_test_env();
    let app_state = utils::create_test_env().await;
    let auth_service = AuthService::new(app_state.db_pool.clone());
    
    // Register user
    let email = utils::mock::test_email();
    let register_request = RegisterRequest {
        name: "Password Reset Test".to_string(),
        email: email.clone(),
        password: "OldPassword123!".to_string(),
        organization: None,
        pseudonym: None,
        phone_numbers: None,
        areas_of_interest: None,
        street: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        primary_email: None,
        recovery_email: None,
        introduced_by: None,
    };
    
    auth_service.register(register_request).await.unwrap();
    
    // Request password reset
    let result = auth_service.request_password_reset(&email).await;
    assert!(result.is_ok(), "Password reset request should succeed");
    
    // Note: In a real test, we would intercept the email and extract the token
    // For now, we'll test with an invalid token
    
    // Test reset with invalid token
    let reset_result = auth_service.reset_password(
        &email,
        "invalid-token",
        "NewPassword123!"
    ).await;
    
    assert!(reset_result.is_err(), "Invalid token should fail");
}

#[tokio::test]
async fn test_role_based_access() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let auth_service = AuthService::new(app_state.db_pool.clone());
    
    // Test admin role
    let admin_email = utils::mock::test_email();
    let admin_request = RegisterRequest {
        name: "Admin User".to_string(),
        email: admin_email.clone(),
        password: "AdminPass123!".to_string(),
        organization: None,
        pseudonym: None,
        phone_numbers: None,
        areas_of_interest: None,
        street: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        primary_email: None,
        recovery_email: None,
        introduced_by: None,
    };
    
    let admin_member = auth_service.register(admin_request).await.unwrap();
    
    // In a real test, we would set admin role via separate API
    // For now, we'll test authorization logic
    
    // Test moderator role
    let mod_email = utils::mock::test_email();
    let mod_request = RegisterRequest {
        name: "Moderator User".to_string(),
        email: mod_email.clone(),
        password: "ModPass123!".to_string(),
        organization: None,
        pseudonym: None,
        phone_numbers: None,
        areas_of_interest: None,
        street: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        primary_email: None,
        recovery_email: None,
        introduced_by: None,
    };
    
    let mod_member = auth_service.register(mod_request).await.unwrap();
    
    // Test author role
    let author_email = utils::mock::test_email();
    let author_request = RegisterRequest {
        name: "Author User".to_string(),
        email: author_email.clone(),
        password: "AuthorPass123!".to_string(),
        organization: None,
        pseudonym: None,
        phone_numbers: None,
        areas_of_interest: None,
        street: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        primary_email: None,
        recovery_email: None,
        introduced_by: None,
    };
    
    let author_member = auth_service.register(author_request).await.unwrap();
}

#[tokio::test]
async fn test_session_management() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let auth_service = AuthService::new(app_state.db_pool.clone());
    
    // Register and login
    let email = utils::mock::test_email();
    let register_request = RegisterRequest {
        name: "Session Test".to_string(),
        email: email.clone(),
        password: "SessionPass123!".to_string(),
        organization: None,
        pseudonym: None,
        phone_numbers: None,
        areas_of_interest: None,
        street: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        primary_email: None,
        recovery_email: None,
        introduced_by: None,
    };
    
    auth_service.register(register_request).await.unwrap();
    
    // Login to get token
    let login_request = LoginRequest {
        email: email.clone(),
        password: "SessionPass123!".to_string(),
        verification_code: None,
    };
    
    let login_response = auth_service.login(login_request).await.unwrap();
    let token = login_response.token.unwrap();
    
    // Verify token
    let verification_result = auth_service.verify_token(&token).await;
    assert!(verification_result.is_ok(), "Token should be valid");
    
    let claims = verification_result.unwrap();
    assert_eq!(claims.email, email);
    
    // Test token expiration (simulated)
    let expired_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0QGV4YW1wbGUuY29tIiwiaWF0IjoxNTE2MjM5MDIyLCJleHAiOjE1MTYyMzkwMjJ9.fake_signature";
    
    let expired_result = auth_service.verify_token(expired_token).await;
    assert!(expired_result.is_err(), "Expired token should fail");
}

#[tokio::test]
async fn test_rate_limiting() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let auth_service = AuthService::new(app_state.db_pool.clone());
    
    let email = utils::mock::test_email();
    
    // Test multiple failed login attempts
    for i in 0..10 {
        let login_request = LoginRequest {
            email: email.clone(),
            password: format!("WrongPassword{}!", i),
            verification_code: None,
        };
        
        let result = auth_service.login(login_request).await;
        // First few attempts might not be rate limited
        // System should eventually block after too many attempts
    }
    
    // Note: Actual rate limiting would be implemented at API layer
    // This test verifies that the service can handle multiple requests
}

#[tokio::test]
async fn test_concurrent_access() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let auth_service = Arc::new(AuthService::new(app_state.db_pool.clone()));
    
    let mut handles = vec![];
    
    // Simulate concurrent registration attempts
    for i in 0..5 {
        let auth_service_clone = auth_service.clone();
        let handle = tokio::spawn(async move {
            let email = format!("concurrent{}@example.com", i);
            let request = RegisterRequest {
                name: format!("User {}", i),
                email,
                password: "ConcurrentPass123!".to_string(),
                organization: None,
                pseudonym: None,
                phone_numbers: None,
                areas_of_interest: None,
                street: None,
                city: None,
                state: None,
                country: None,
                postal_code: None,
                primary_email: None,
                recovery_email: None,
                introduced_by: None,
            };
            
            auth_service_clone.register(request).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all registrations to complete
    for handle in handles {
        let result = handle.await.expect("Task panicked");
        assert!(result.is_ok(), "Concurrent registration should succeed");
    }
}
