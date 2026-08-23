//! Donation-related integration tests

use crate::tests::utils;
use crate::app::AppState;
use crate::domain::donation::{Donation, DonationDistribution};
use crate::domain::member::Member;
use crate::domain::text::Text;
use crate::domain::charity::Charity;
use crate::services::donation_service::DonationService;
use crate::services::member_service::MemberService;
use crate::services::text_service::TextService;
use crate::services::charity_service::CharityService;
use chrono::Utc;
use rust_decimal::Decimal;

#[tokio::test]
async fn test_donation_creation_flow() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let donation_service = DonationService::new(app_state.db_pool.clone());
    let member_service = MemberService::new(app_state.db_pool.clone());
    let text_service = TextService::new(app_state.db_pool.clone());
    let charity_service = CharityService::new(app_state.db_pool.clone());
    
    // Create test member
    let member = member_service.create_member(
        "Donor Test".to_string(),
        utils::mock::test_email(),
        "TestPass123!".to_string(),
        None,
        None,
        None,
        None,
        None
    ).await.unwrap();
    
    // Create test author (text owner)
    let author = member_service.create_member(
        "Author Test".to_string(),
        utils::mock::test_email(),
        "AuthorPass123!".to_string(),
        None,
        None,
        None,
        None,
        None
    ).await.unwrap();
    
    // Create test text
    let text = text_service.create_text(
        author.member_id,
        "Test Text for Donation".to_string(),
        Some("Test abstract".to_string()),
        "Technology".to_string(),
        vec!["test".to_string(), "donation".to_string()],
        1,
        Utc::now().date_naive(),
        "published".to_string()
    ).await.unwrap();
    
    // Create test charity
    let charity = charity_service.create_charity(
        "Test Charity".to_string(),
        Some("Test charity description".to_string()),
        "Help testers".to_string(),
        "Testland".to_string(),
        format!("REG-{}", utils::mock::test_orcid()),
        "active".to_string()
    ).await.unwrap();
    
    // Test valid donation
    let donation_request = Donation {
        donation_id: 0, // Will be set by DB
        member_id: member.member_id,
        text_id: text.text_id,
        charity_id: charity.charity_id,
        amount: Decimal::from(100),
        date: Utc::now(),
        currency: "USD".to_string(),
        payment_method: "credit_card".to_string(),
        transaction_id: donation_service.generate_transaction_id(),
        charity_pct: 60,
        cfp_pct: 20,
        author_pct: 20,
    };
    
    let result = donation_service.create_donation(donation_request).await;
    assert!(result.is_ok(), "Donation creation should succeed");
    
    let created_donation = result.unwrap();
    assert_eq!(created_donation.amount, Decimal::from(100));
    assert_eq!(created_donation.charity_pct, 60);
    assert_eq!(created_donation.charity_pct + created_donation.cfp_pct + created_donation.author_pct, 100);
    
    // Test donation with charity percentage < 60%
    let invalid_donation = Donation {
        charity_pct: 50, // Should be >= 60
        cfp_pct: 25,
        author_pct: 25,
        ..donation_request
    };
    
    let result = donation_service.create_donation(invalid_donation).await;
    assert!(result.is_err(), "Donation with charity < 60% should fail");
    
    // Test donation with percentages not summing to 100
    let invalid_sum_donation = Donation {
        charity_pct: 60,
        cfp_pct: 30,
        author_pct: 20, // Sum = 110
        ..donation_request
    };
    
    let result = donation_service.create_donation(invalid_sum_donation).await;
    assert!(result.is_err(), "Donation with percentages not summing to 100 should fail");
    
    // Test minimum donation amount ($1.00)
    let small_donation = Donation {
        amount: Decimal::from_f64(0.50).unwrap(), // Below minimum
        ..donation_request
    };
    
    let result = donation_service.create_donation(small_donation).await;
    assert!(result.is_err(), "Donation below $1.00 should fail");
}

#[tokio::test]
async fn test_donation_distribution() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let donation_service = DonationService::new(app_state.db_pool.clone());
    
    // Create test entities (simplified)
    let member_id = 1;
    let text_id = 1;
    let charity_id = 1;
    
    // Create multiple donations
    let donations = vec![
        Donation {
            donation_id: 0,
            member_id,
            text_id,
            charity_id,
            amount: Decimal::from(100),
            date: Utc::now(),
            currency: "USD".to_string(),
            payment_method: "credit_card".to_string(),
            transaction_id: donation_service.generate_transaction_id(),
            charity_pct: 60,
            cfp_pct: 20,
            author_pct: 20,
        },
        Donation {
            donation_id: 0,
            member_id,
            text_id,
            charity_id,
            amount: Decimal::from(200),
            date: Utc::now(),
            currency: "USD".to_string(),
            payment_method: "paypal".to_string(),
            transaction_id: donation_service.generate_transaction_id(),
            charity_pct: 70,
            cfp_pct: 20,
            author_pct: 10,
        },
    ];
    
    // Create donations
    for donation in donations {
        donation_service.create_donation(donation).await.unwrap();
    }
    
    // Test charity total calculation
    let charity_total = donation_service.get_charity_total(charity_id).await;
    assert!(charity_total.is_ok());
    // Charity gets: 60% of 100 = 60, 70% of 200 = 140, total = 200
    assert_eq!(charity_total.unwrap(), Decimal::from(200));
    
    // Test text total donations
    let text_total = donation_service.get_text_total_donations(text_id).await;
    assert!(text_total.is_ok());
    // Text (author + CFP) gets: 40% of 100 = 40, 30% of 200 = 60, total = 100
    assert_eq!(text_total.unwrap(), Decimal::from(100));
    
    // Test member donation history
    let member_history = donation_service.get_member_donations(member_id).await;
    assert!(member_history.is_ok());
    let history = member_history.unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history.iter().map(|d| d.amount).sum::<Decimal>(), Decimal::from(300));
}

#[tokio::test]
async fn test_currency_handling() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let donation_service = DonationService::new(app_state.db_pool.clone());
    
    let member_id = 1;
    let text_id = 1;
    let charity_id = 1;
    
    // Test different currencies
    let currencies = vec!["USD", "EUR", "GBP", "JPY"];
    
    for currency in currencies {
        let donation = Donation {
            donation_id: 0,
            member_id,
            text_id,
            charity_id,
            amount: Decimal::from(50),
            date: Utc::now(),
            currency: currency.to_string(),
            payment_method: "credit_card".to_string(),
            transaction_id: donation_service.generate_transaction_id(),
            charity_pct: 60,
            cfp_pct: 20,
            author_pct: 20,
        };
        
        let result = donation_service.create_donation(donation).await;
        assert!(result.is_ok(), format!("Donation in {} should succeed", currency));
    }
    
    // Test invalid currency
    let invalid_donation = Donation {
        donation_id: 0,
        member_id,
        text_id,
        charity_id,
        amount: Decimal::from(50),
        date: Utc::now(),
        currency: "INVALID".to_string(), // Not 3 characters
        payment_method: "credit_card".to_string(),
        transaction_id: donation_service.generate_transaction_id(),
        charity_pct: 60,
        cfp_pct: 20,
        author_pct: 20,
    };
    
    let result = donation_service.create_donation(invalid_donation).await;
    assert!(result.is_err(), "Invalid currency should fail");
}

#[tokio::test]
async fn test_payment_method_validation() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let donation_service = DonationService::new(app_state.db_pool.clone());
    
    let member_id = 1;
    let text_id = 1;
    let charity_id = 1;
    
    // Test valid payment methods
    let valid_methods = vec!["credit_card", "paypal", "bank_transfer", "crypto"];
    
    for method in valid_methods {
        let donation = Donation {
            donation_id: 0,
            member_id,
            text_id,
            charity_id,
            amount: Decimal::from(25),
            date: Utc::now(),
            currency: "USD".to_string(),
            payment_method: method.to_string(),
            transaction_id: donation_service.generate_transaction_id(),
            charity_pct: 60,
            cfp_pct: 20,
            author_pct: 20,
        };
        
        let result = donation_service.create_donation(donation).await;
        assert!(result.is_ok(), format!("Payment method {} should be valid", method));
    }
    
    // Test invalid payment method
    let invalid_donation = Donation {
        donation_id: 0,
        member_id,
        text_id,
        charity_id,
        amount: Decimal::from(25),
        date: Utc::now(),
        currency: "USD".to_string(),
        payment_method: "invalid_method".to_string(),
        transaction_id: donation_service.generate_transaction_id(),
        charity_pct: 60,
        cfp_pct: 20,
        author_pct: 20,
    };
    
    let result = donation_service.create_donation(invalid_donation).await;
    assert!(result.is_err(), "Invalid payment method should fail");
}

#[tokio::test]
async fn test_transaction_id_uniqueness() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let donation_service = DonationService::new(app_state.db_pool.clone());
    
    let member_id = 1;
    let text_id = 1;
    let charity_id = 1;
    
    // Generate a transaction ID
    let transaction_id = donation_service.generate_transaction_id();
    
    // First donation with this transaction ID
    let donation1 = Donation {
        donation_id: 0,
        member_id,
        text_id,
        charity_id,
        amount: Decimal::from(100),
        date: Utc::now(),
        currency: "USD".to_string(),
        payment_method: "credit_card".to_string(),
        transaction_id: transaction_id.clone(),
        charity_pct: 60,
        cfp_pct: 20,
        author_pct: 20,
    };
    
    let result1 = donation_service.create_donation(donation1).await;
    assert!(result1.is_ok(), "First donation should succeed");
    
    // Second donation with same transaction ID should fail
    let donation2 = Donation {
        donation_id: 0,
        member_id: 2, // Different member
        text_id,
        charity_id,
        amount: Decimal::from(50),
        date: Utc::now(),
        currency: "USD".to_string(),
        payment_method: "paypal".to_string(),
        transaction_id: transaction_id.clone(),
        charity_pct: 60,
        cfp_pct: 20,
        author_pct: 20,
    };
    
    let result2 = donation_service.create_donation(donation2).await;
    assert!(result2.is_err(), "Duplicate transaction ID should fail");
}

#[tokio::test]
async fn test_donation_statistics() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let donation_service = DonationService::new(app_state.db_pool.clone());
    
    // Create multiple donations over time
    let mut total_amount = Decimal::from(0);
    
    for i in 1..=10 {
        let donation = Donation {
            donation_id: 0,
            member_id: i,
            text_id: 1,
            charity_id: 1,
            amount: Decimal::from(i * 10), // 10, 20, ..., 100
            date: Utc::now(),
            currency: "USD".to_string(),
            payment_method: "credit_card".to_string(),
            transaction_id: donation_service.generate_transaction_id(),
            charity_pct: 60,
            cfp_pct: 20,
            author_pct: 20,
        };
        
        donation_service.create_donation(donation).await.unwrap();
        total_amount += Decimal::from(i * 10);
    }
    
    // Test statistics
    let stats = donation_service.get_donation_statistics().await;
    assert!(stats.is_ok());
    
    let stats = stats.unwrap();
    assert_eq!(stats.total_donations, 10);
    assert_eq!(stats.total_amount, total_amount);
    assert_eq!(stats.average_donation, total_amount / Decimal::from(10));
    
    // Test top donors
    let top_donors = donation_service.get_top_donors(5).await;
    assert!(stats.is_ok());
    let donors = top_donors.unwrap();
    assert_eq!(donors.len(), 5);
    // Member 10 donated 100, should be first
    assert_eq!(donors[0].member_id, 10);
    
    // Test charity rankings
    let charity_rankings = donation_service.get_charity_rankings().await;
    assert!(charity_rankings.is_ok());
}

#[tokio::test]
async fn test_refund_processing() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let donation_service = DonationService::new(app_state.db_pool.clone());
    
    let member_id = 1;
    let text_id = 1;
    let charity_id = 1;
    
    // Create a donation
    let donation = Donation {
        donation_id: 0,
        member_id,
        text_id,
        charity_id,
        amount: Decimal::from(100),
        date: Utc::now(),
        currency: "USD".to_string(),
        payment_method: "credit_card".to_string(),
        transaction_id: donation_service.generate_transaction_id(),
        charity_pct: 60,
        cfp_pct: 20,
        author_pct: 20,
    };
    
    let created = donation_service.create_donation(donation).await.unwrap();
    
    // Test refund
    let refund_result = donation_service.process_refund(created.donation_id, "customer_request").await;
    assert!(refund_result.is_ok(), "Refund should succeed");
    
    let refunded = refund_result.unwrap();
    assert_eq!(refunded.amount, Decimal::from(-100)); // Negative amount for refund
    
    // Test double refund should fail
    let double_refund = donation_service.process_refund(created.donation_id, "duplicate").await;
    assert!(double_refund.is_err(), "Double refund should fail");
    
    // Test refund on non-existent donation
    let fake_refund = donation_service.process_refund(9999, "test").await;
    assert!(fake_refund.is_err(), "Refund on non-existent donation should fail");
}

#[tokio::test]
async fn test_donation_webhook_handling() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    let donation_service = DonationService::new(app_state.db_pool.clone());
    
    // Test webhook signature verification
    let payload = r#"{"transaction_id": "test_123", "status": "completed"}"#;
    let secret = "webhook_secret";
    
    let signature = donation_service.generate_webhook_signature(payload, secret).await;
    assert!(signature.is_ok());
    
    let sig = signature.unwrap();
    
    // Verify signature
    let is_valid = donation_service.verify_webhook_signature(payload, &sig, secret).await;
    assert!(is_valid.is_ok() && is_valid.unwrap(), "Signature should be valid");
    
    // Test tampered payload
    let tampered_payload = r#"{"transaction_id": "test_123", "status": "failed"}"#;
    let is_tampered_valid = donation_service.verify_webhook_signature(tampered_payload, &sig, secret).await;
    assert!(is_tampered_valid.is_ok() && !is_tampered_valid.unwrap(), "Tampered payload should fail");
}
