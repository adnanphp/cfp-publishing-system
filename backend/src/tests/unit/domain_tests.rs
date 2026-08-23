//! Domain model unit tests

use crate::domain::member::Member;
use crate::domain::text::Text;
use crate::domain::donation::Donation;
use crate::domain::comment::Comment;
use crate::domain::plagiarism::PlagiarismCase;
use crate::utils::validation;
use crate::utils::datetime::CFPDateUtils;
use chrono::{Utc, Duration};
use rust_decimal::Decimal;

#[test]
fn test_member_domain_model() {
    // Test member creation with valid data
    let member = Member {
        member_id: 1,
        name: "Test User".to_string(),
        organization: Some("Test Org".to_string()),
        pseudonym: Some("tester".to_string()),
        primary_email: Some("test@example.com".to_string()),
        recovery_email: Some("recovery@example.com".to_string()),
        password_hash: "$argon2id$v=19$m=65536,t=3,p=4$salt$hash".to_string(),
        verification_matrix: Some("ABCDEF-GHIJKL-MNOPQR".to_string()),
        matrix_expiry: Some(CFPDateUtils::calculate_matrix_expiry()),
        join_date: Utc::now().date_naive(),
        status: "active".to_string(),
        download_limit: 10,
        street: Some("123 Test St".to_string()),
        city: Some("Test City".to_string()),
        state: Some("TS".to_string()),
        country: Some("Testland".to_string()),
        postal_code: Some("12345".to_string()),
        phone_numbers: Some(vec!["+1234567890".to_string()]),
        areas_of_interest: Some(vec!["AI".to_string(), "ML".to_string()]),
        introduced_by: None,
    };
    
    assert_eq!(member.member_id, 1);
    assert_eq!(member.name, "Test User");
    assert!(member.verification_matrix.is_some());
    assert_eq!(member.status, "active");
    
    // Test derived status calculation
    let pending_member = Member {
        status: "pending".to_string(),
        ..member.clone()
    };
    
    assert_eq!(pending_member.is_verified(), false);
    
    let active_member = Member {
        status: "active".to_string(),
        ..member.clone()
    };
    
    assert_eq!(active_member.is_verified(), true);
    
    // Test download limit calculation
    let donor_member = Member {
        status: "active".to_string(),
        ..member.clone()
    };
    
    // Assuming member has donations
    assert!(donor_member.get_download_limit() >= 1);
    
    // Test verification matrix expiry
    let expired_member = Member {
        matrix_expiry: Some((Utc::now() - Duration::days(15)).date_naive()),
        ..member.clone()
    };
    
    assert!(expired_member.is_verification_expired());
    
    let valid_member = Member {
        matrix_expiry: Some((Utc::now() + Duration::days(7)).date_naive()),
        ..member
    };
    
    assert!(!valid_member.is_verification_expired());
}

#[test]
fn test_text_domain_model() {
    let text = Text {
        text_id: 1,
        title: "Test Text Title".to_string(),
        abstract: Some("This is a test abstract for the text.".to_string()),
        topic: "Technology".to_string(),
        keywords: Some(vec!["test".to_string(), "technology".to_string(), "sample".to_string()]),
        version: 1,
        upload_date: Utc::now().date_naive(),
        status: "published".to_string(),
        download_count: 0,
        total_donations: Decimal::from(0),
        avg_rating: None,
        author_orcid: "0000-0002-1825-0097".to_string(),
    };
    
    assert_eq!(text.text_id, 1);
    assert_eq!(text.title, "Test Text Title");
    assert_eq!(text.version, 1);
    assert_eq!(text.status, "published");
    
    // Test version validation
    assert!(text.is_valid_version());
    
    let invalid_version_text = Text {
        version: 0,
        ..text.clone()
    };
    
    assert!(!invalid_version_text.is_valid_version());
    
    // Test keyword count limit
    let many_keywords_text = Text {
        keywords: Some((0..15).map(|i| format!("keyword{}", i)).collect()),
        ..text.clone()
    };
    
    assert!(!many_keywords_text.is_valid_keyword_count());
    
    // Test status transitions
    assert!(text.can_transition_to("archived"));
    assert!(!text.can_transition_to("draft")); // Cannot go back to draft from published
    
    // Test download count increment
    let mut downloadable_text = text.clone();
    downloadable_text.increment_download_count();
    assert_eq!(downloadable_text.download_count, 1);
    
    // Test rating calculation
    let mut rated_text = text.clone();
    rated_text.add_rating(4);
    rated_text.add_rating(5);
    assert_eq!(rated_text.avg_rating, Some(4.5));
    
    // Test donation addition
    let mut donated_text = text;
    donated_text.add_donation(Decimal::from(100));
    assert_eq!(donated_text.total_donations, Decimal::from(100));
}

#[test]
fn test_donation_domain_model() {
    let donation = Donation {
        donation_id: 1,
        member_id: 1,
        text_id: 1,
        charity_id: 1,
        amount: Decimal::from(100),
        date: Utc::now(),
        currency: "USD".to_string(),
        payment_method: "credit_card".to_string(),
        transaction_id: "txn_123456789".to_string(),
        charity_pct: 60,
        cfp_pct: 20,
        author_pct: 20,
    };
    
    assert_eq!(donation.donation_id, 1);
    assert_eq!(donation.amount, Decimal::from(100));
    assert_eq!(donation.currency, "USD");
    
    // Test percentage validation
    assert!(donation.is_valid_percentage_distribution());
    assert_eq!(donation.charity_pct + donation.cfp_pct + donation.author_pct, 100);
    assert!(donation.charity_pct >= 60);
    
    let invalid_donation = Donation {
        charity_pct: 50, // < 60
        cfp_pct: 25,
        author_pct: 25,
        ..donation.clone()
    };
    
    assert!(!invalid_donation.is_valid_percentage_distribution());
    
    // Test amount validation
    assert!(donation.is_valid_amount());
    
    let small_donation = Donation {
        amount: Decimal::from_f64(0.50).unwrap(),
        ..donation.clone()
    };
    
    assert!(!small_donation.is_valid_amount());
    
    // Test distribution calculation
    let distribution = donation.calculate_distribution();
    assert_eq!(distribution.charity_amount, Decimal::from(60));
    assert_eq!(distribution.cfp_amount, Decimal::from(20));
    assert_eq!(distribution.author_amount, Decimal::from(20));
    
    // Test currency validation
    assert!(donation.is_valid_currency());
    
    let invalid_currency_donation = Donation {
        currency: "INVALID".to_string(),
        ..donation
    };
    
    assert!(!invalid_currency_donation.is_valid_currency());
}

#[test]
fn test_comment_domain_model() {
    let comment = Comment {
        comment_id: 1,
        member_id: 1,
        text_id: 1,
        parent_comment_id: None,
        content: "This is a test comment with at least 10 characters.".to_string(),
        date: Utc::now(),
        is_public: true,
        rating: Some(4),
        status: "active".to_string(),
    };
    
    assert_eq!(comment.comment_id, 1);
    assert_eq!(comment.member_id, 1);
    assert_eq!(comment.text_id, 1);
    assert!(comment.is_public);
    
    // Test content validation
    assert!(comment.is_valid_content());
    
    let short_comment = Comment {
        content: "short".to_string(),
        ..comment.clone()
    };
    
    assert!(!short_comment.is_valid_content());
    
    // Test rating validation
    assert!(comment.is_valid_rating());
    
    let invalid_rating_comment = Comment {
        rating: Some(6), // > 5
        ..comment.clone()
    };
    
    assert!(!invalid_rating_comment.is_valid_rating());
    
    let no_rating_comment = Comment {
        rating: None,
        ..comment.clone()
    };
    
    assert!(no_rating_comment.is_valid_rating());
    
    // Test reply functionality
    let reply = Comment {
        comment_id: 2,
        parent_comment_id: Some(1),
        ..comment.clone()
    };
    
    assert!(reply.is_reply());
    assert_eq!(reply.parent_comment_id, Some(1));
    
    // Test status transitions
    assert!(comment.can_be_flagged());
    assert!(comment.can_be_removed());
    
    let flagged_comment = Comment {
        status: "flagged".to_string(),
        ..comment.clone()
    };
    
    assert!(!flagged_comment.can_be_flagged()); // Already flagged
    
    // Test date validation
    assert!(!comment.is_too_old()); // Just created
    
    let old_comment = Comment {
        date: Utc::now() - Duration::days(365),
        ..comment
    };
    
    assert!(old_comment.is_too_old());
}

#[test]
fn test_plagiarism_case_domain_model() {
    let plagiarism_case = PlagiarismCase {
        case_id: 1,
        committee_id: 1,
        text_id: 1,
        opened_date: Utc::now().date_naive(),
        description: Some("Potential plagiarism detected".to_string()),
        status: "open".to_string(),
        resolution: None,
        closed_date: None,
    };
    
    assert_eq!(plagiarism_case.case_id, 1);
    assert_eq!(plagiarism_case.committee_id, 1);
    assert_eq!(plagiarism_case.status, "open");
    
    // Test status transitions
    assert!(plagiarism_case.can_transition_to("under_review"));
    assert!(plagiarism_case.can_transition_to("voting"));
    assert!(!plagiarism_case.can_transition_to("closed")); // Needs resolution first
    
    // Test voting eligibility
    let voting_case = PlagiarismCase {
        status: "voting".to_string(),
        ..plagiarism_case.clone()
    };
    
    assert!(voting_case.is_in_voting_period());
    
    // Test closed case
    let closed_case = PlagiarismCase {
        status: "closed".to_string(),
        resolution: Some("plagiarized".to_string()),
        closed_date: Some(Utc::now().date_naive()),
        ..plagiarism_case.clone()
    };
    
    assert!(!closed_case.is_in_voting_period());
    assert!(closed_case.is_resolved());
    assert_eq!(closed_case.resolution, Some("plagiarized".to_string()));
    
    // Test appeal eligibility
    assert!(closed_case.can_be_appealed());
    
    let not_plagiarized_case = PlagiarismCase {
        resolution: Some("not_plagiarized".to_string()),
        ..closed_case.clone()
    };
    
    assert!(!not_plagiarized_case.can_be_appealed()); // Can only appeal plagiarized verdicts
    
    // Test duration calculation
    let old_case = PlagiarismCase {
        opened_date: (Utc::now() - Duration::days(20)).date_naive(),
        ..plagiarism_case.clone()
    };
    
    assert!(old_case.is_voting_period_expired());
    
    // Test description validation
    assert!(plagiarism_case.has_valid_description());
    
    let empty_description_case = PlagiarismCase {
        description: Some("".to_string()),
        ..plagiarism_case
    };
    
    assert!(!empty_description_case.has_valid_description());
}

#[test]
fn test_domain_model_relationships() {
    // Test member-text relationship through downloads
    let member = Member {
        member_id: 1,
        name: "Test Member".to_string(),
        organization: None,
        pseudonym: None,
        primary_email: None,
        recovery_email: None,
        password_hash: "hash".to_string(),
        verification_matrix: None,
        matrix_expiry: None,
        join_date: Utc::now().date_naive(),
        status: "active".to_string(),
        download_limit: 5,
        street: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        phone_numbers: None,
        areas_of_interest: None,
        introduced_by: None,
    };
    
    let text = Text {
        text_id: 1,
        title: "Test Text".to_string(),
        abstract: None,
        topic: "Test".to_string(),
        keywords: None,
        version: 1,
        upload_date: Utc::now().date_naive(),
        status: "published".to_string(),
        download_count: 0,
        total_donations: Decimal::from(0),
        avg_rating: None,
        author_orcid: "0000-0001-2345-6789".to_string(),
    };
    
    // Simulate download
    assert!(member.can_download());
    text.increment_download_count();
    assert_eq!(text.download_count, 1);
    
    // Test donation relationship
    let donation = Donation {
        donation_id: 1,
        member_id: member.member_id,
        text_id: text.text_id,
        charity_id: 1,
        amount: Decimal::from(50),
        date: Utc::now(),
        currency: "USD".to_string(),
        payment_method: "credit_card".to_string(),
        transaction_id: "txn_123".to_string(),
        charity_pct: 60,
        cfp_pct: 20,
        author_pct: 20,
    };
    
    text.add_donation(donation.amount);
    assert_eq!(text.total_donations, Decimal::from(50));
    
    // Test comment relationship
    let comment = Comment {
        comment_id: 1,
        member_id: member.member_id,
        text_id: text.text_id,
        parent_comment_id: None,
        content: "Great text!".to_string(),
        date: Utc::now(),
        is_public: true,
        rating: Some(5),
        status: "active".to_string(),
    };
    
    text.add_rating(comment.rating.unwrap());
    assert_eq!(text.avg_rating, Some(5.0));
}

#[test]
fn test_domain_model_validation() {
    // Test validation integration
    let email = "test@example.com";
    assert!(validation::validate_email(email).is_ok());
    
    let invalid_email = "not-an-email";
    assert!(validation::validate_email(invalid_email).is_err());
    
    let password = "ValidPass123!";
    assert!(validation::validate_password(password).is_ok());
    
    let weak_password = "weak";
    assert!(validation::validate_password(weak_password).is_err());
    
    let orcid = "0000-0002-1825-0097";
    assert!(validation::validate_orcid(orcid).is_ok());
    
    let invalid_orcid = "1234-5678";
    assert!(validation::validate_orcid(invalid_orcid).is_err());
    
    let rating = 3;
    assert!(validation::validate_rating(rating).is_ok());
    
    let invalid_rating = 6;
    assert!(validation::validate_rating(invalid_rating).is_err());
    
    let keywords = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    assert!(validation::validate_keywords(&keywords).is_ok());
    
    let many_keywords: Vec<String> = (0..15).map(|i| format!("keyword{}", i)).collect();
    assert!(validation::validate_keywords(&many_keywords).is_err());
    
    let comment_content = "This is a valid comment with enough characters.";
    assert!(validation::validate_comment_content(comment_content).is_ok());
    
    let short_content = "short";
    assert!(validation::validate_comment_content(short_content).is_err());
}
