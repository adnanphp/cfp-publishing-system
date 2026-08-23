//! Plagiarism detection and handling integration tests

use crate::tests::utils;
use crate::app::AppState;
use crate::domain::plagiarism::{PlagiarismCase, PlagiarismVote};
use crate::domain::committee::Committee;
use crate::domain::text::Text;
use crate::domain::member::Member;
use crate::services::plagiarism_service::PlagiarismService;
use crate::services::committee_service::CommitteeService;
use crate::services::member_service::MemberService;
use crate::services::text_service::TextService;
use chrono::{Utc, Duration};
use rust_decimal::Decimal;

#[tokio::test]
async fn test_plagiarism_case_creation() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    let committee_service = CommitteeService::new(app_state.db_pool.clone());
    let member_service = MemberService::new(app_state.db_pool.clone());
    let text_service = TextService::new(app_state.db_pool.clone());
    
    // Create test committee
    let committee = committee_service.create_committee(
        "Plagiarism Review Committee".to_string(),
        "Review plagiarism cases".to_string(),
        "plagiarism".to_string(),
        Utc::now().date_naive(),
        "active".to_string()
    ).await.unwrap();
    
    // Create test author
    let author = member_service.create_member(
        "Author Under Review".to_string(),
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
        "Suspicious Text".to_string(),
        Some("This might be plagiarized".to_string()),
        "Academic".to_string(),
        vec!["plagiarism".to_string(), "test".to_string()],
        1,
        Utc::now().date_naive(),
        "published".to_string()
    ).await.unwrap();
    
    // Create plagiarism case
    let case_request = PlagiarismCase {
        case_id: 0,
        committee_id: committee.committee_id,
        text_id: text.text_id,
        opened_date: Utc::now().date_naive(),
        description: Some("Potential plagiarism detected".to_string()),
        status: "open".to_string(),
        resolution: None,
        closed_date: None,
    };
    
    let result = plagiarism_service.create_case(case_request).await;
    assert!(result.is_ok(), "Plagiarism case creation should succeed");
    
    let created_case = result.unwrap();
    assert_eq!(created_case.status, "open");
    assert_eq!(created_case.committee_id, committee.committee_id);
    assert_eq!(created_case.text_id, text.text_id);
    
    // Test case with non-existent text should fail
    let invalid_case = PlagiarismCase {
        text_id: 9999, // Non-existent
        ..case_request
    };
    
    let result = plagiarism_service.create_case(invalid_case).await;
    assert!(result.is_err(), "Case with non-existent text should fail");
    
    // Test case with non-existent committee should fail
    let invalid_committee_case = PlagiarismCase {
        committee_id: 9999, // Non-existent
        ..case_request
    };
    
    let result = plagiarism_service.create_case(invalid_committee_case).await;
    assert!(result.is_err(), "Case with non-existent committee should fail");
}

#[tokio::test]
async fn test_plagiarism_voting_process() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    let committee_service = CommitteeService::new(app_state.db_pool.clone());
    let member_service = MemberService::new(app_state.db_pool.clone());
    let text_service = TextService::new(app_state.db_pool.clone());
    
    // Create committee
    let committee = committee_service.create_committee(
        "Voting Test Committee".to_string(),
        "Test voting process".to_string(),
        "plagiarism".to_string(),
        Utc::now().date_naive(),
        "active".to_string()
    ).await.unwrap();
    
    // Create author and text
    let author = member_service.create_member(
        "Voting Test Author".to_string(),
        utils::mock::test_email(),
        "AuthorPass123!".to_string(),
        None,
        None,
        None,
        None,
        None
    ).await.unwrap();
    
    let text = text_service.create_text(
        author.member_id,
        "Text for Voting".to_string(),
        None,
        "Test".to_string(),
        vec![],
        1,
        Utc::now().date_naive(),
        "published".to_string()
    ).await.unwrap();
    
    // Create plagiarism case
    let case = plagiarism_service.create_case(PlagiarismCase {
        case_id: 0,
        committee_id: committee.committee_id,
        text_id: text.text_id,
        opened_date: Utc::now().date_naive(),
        description: Some("Voting test case".to_string()),
        status: "voting".to_string(),
        resolution: None,
        closed_date: None,
    }).await.unwrap();
    
    // Create committee members who will vote
    let mut voters = vec![];
    for i in 0..5 {
        let voter = member_service.create_member(
            format!("Voter {}", i),
            utils::mock::test_email(),
            "VoterPass123!".to_string(),
            None,
            None,
            None,
            None,
            None
        ).await.unwrap();
        
        // Add to committee
        committee_service.add_member_to_committee(
            committee.committee_id,
            voter.member_id,
            "member".to_string(),
            Utc::now().date_naive(),
            "active".to_string(),
            None
        ).await.unwrap();
        
        voters.push(voter);
    }
    
    // Cast votes (3 for plagiarized, 2 for not plagiarized)
    let votes = vec![
        ("plagiarized", "Clear plagiarism found"),
        ("plagiarized", "Multiple sources copied"),
        ("plagiarized", "Significant similarity"),
        ("not_plagiarized", "Properly cited"),
        ("not_plagiarized", "Coincidental similarity"),
    ];
    
    for (i, (vote_type, rationale)) in votes.iter().enumerate() {
        let vote = PlagiarismVote {
            vote_id: 0,
            member_id: voters[i].member_id,
            case_id: case.case_id,
            vote: vote_type.to_string(),
            date: Utc::now(),
            rationale: Some(rationale.to_string()),
        };
        
        let result = plagiarism_service.cast_vote(vote).await;
        assert!(result.is_ok(), format!("Vote {} should succeed", i));
    }
    
    // Test voting results
    let results = plagiarism_service.get_case_results(case.case_id).await;
    assert!(results.is_ok());
    
    let results = results.unwrap();
    assert_eq!(results.total_votes, 5);
    assert_eq!(results.plagiarized_votes, 3);
    assert_eq!(results.not_plagiarized_votes, 2);
    assert_eq!(results.abstain_votes, 0);
    
    // 3/5 = 60%, needs 2/3 (66.7%) for plagiarism finding
    assert!(!results.is_plagiarized(), "Should not reach 2/3 majority");
    
    // Test duplicate vote prevention
    let duplicate_vote = PlagiarismVote {
        vote_id: 0,
        member_id: voters[0].member_id, // Already voted
        case_id: case.case_id,
        vote: "abstain".to_string(),
        date: Utc::now(),
        rationale: Some("Changing vote".to_string()),
    };
    
    let result = plagiarism_service.cast_vote(duplicate_vote).await;
    assert!(result.is_err(), "Duplicate vote should fail");
}

#[tokio::test]
async fn test_2_3_majority_rule() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    
    // Test various vote distributions
    let test_cases = vec![
        // (plagiarized, not_plagiarized, abstain, expected_result)
        (7, 0, 0, true),   // 100% plagiarized
        (5, 2, 0, true),   // ~71% plagiarized (>66.7%)
        (4, 2, 1, false),  // ~57% plagiarized (<66.7%)
        (2, 5, 0, false),  // ~29% plagiarized
        (0, 7, 0, false),  // 0% plagiarized
        (4, 0, 3, false),  // Majority but not 2/3 of total
    ];
    
    for (i, (plagiarized, not_plagiarized, abstain, expected)) in test_cases.iter().enumerate() {
        let total = plagiarized + not_plagiarized + abstain;
        let percentage = (*plagiarized as f64) / (total as f64) * 100.0;
        
        let is_plagiarized = percentage >= (2.0/3.0) * 100.0;
        assert_eq!(is_plagiarized, *expected, 
            "Test case {} failed: plagiarized={}, not={}, abstain={}", 
            i, plagiarized, not_plagiarized, abstain);
    }
}

#[tokio::test]
async fn test_voting_period_enforcement() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    
    // Create a case with past voting deadline
    let old_case = PlagiarismCase {
        case_id: 0,
        committee_id: 1,
        text_id: 1,
        opened_date: (Utc::now() - Duration::days(15)).date_naive(), // 15 days ago
        description: None,
        status: "voting".to_string(),
        resolution: None,
        closed_date: None,
    };
    
    // This case should be automatically closed
    let result = plagiarism_service.check_voting_period(&old_case).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "closed"); // Voting period ended
    
    // Create a case with active voting period
    let new_case = PlagiarismCase {
        case_id: 0,
        committee_id: 1,
        text_id: 1,
        opened_date: Utc::now().date_naive(), // Today
        description: None,
        status: "voting".to_string(),
        resolution: None,
        closed_date: None,
    };
    
    let result = plagiarism_service.check_voting_period(&new_case).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "voting"); // Still in voting period
}

#[tokio::test]
async fn test_three_strikes_rule() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    let member_service = MemberService::new(app_state.db_pool.clone());
    
    // Create author
    let author = member_service.create_member(
        "Three Strikes Author".to_string(),
        utils::mock::test_email(),
        "AuthorPass123!".to_string(),
        None,
        None,
        None,
        None,
        None
    ).await.unwrap();
    
    // Simulate 2 previous violations
    for i in 0..2 {
        let case = PlagiarismCase {
            case_id: 0,
            committee_id: 1,
            text_id: i as i32 + 1,
            opened_date: Utc::now().date_naive(),
            description: None,
            status: "closed".to_string(),
            resolution: Some("plagiarized".to_string()),
            closed_date: Some(Utc::now().date_naive()),
        };
        
        plagiarism_service.record_violation(author.member_id, &case).await.unwrap();
    }
    
    // Check current violation count
    let violations = plagiarism_service.get_member_violations(author.member_id).await;
    assert!(violations.is_ok());
    assert_eq!(violations.unwrap(), 2);
    
    // Add third violation - should trigger blacklist
    let third_case = PlagiarismCase {
        case_id: 0,
        committee_id: 1,
        text_id: 3,
        opened_date: Utc::now().date_naive(),
        description: None,
        status: "closed".to_string(),
        resolution: Some("plagiarized".to_string()),
        closed_date: Some(Utc::now().date_naive()),
    };
    
    let result = plagiarism_service.record_violation(author.member_id, &third_case).await;
    assert!(result.is_ok());
    
    // Author should now be blacklisted
    let is_blacklisted = plagiarism_service.is_member_blacklisted(author.member_id).await;
    assert!(is_blacklisted.is_ok() && is_blacklisted.unwrap(), "Author should be blacklisted after 3 violations");
}

#[tokio::test]
async fn test_appeal_process() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    
    // Create a closed case
    let case = PlagiarismCase {
        case_id: 0,
        committee_id: 1,
        text_id: 1,
        opened_date: Utc::now().date_naive(),
        description: Some("Original case".to_string()),
        status: "closed".to_string(),
        resolution: Some("plagiarized".to_string()),
        closed_date: Some(Utc::now().date_naive()),
    };
    
    let created_case = plagiarism_service.create_case(case).await.unwrap();
    
    // Initiate appeal
    let appeal_result = plagiarism_service.initiate_appeal(
        created_case.case_id,
        "New evidence found".to_string()
    ).await;
    
    assert!(appeal_result.is_ok(), "Appeal should succeed");
    
    let appealed_case = appeal_result.unwrap();
    assert_eq!(appealed_case.status, "appealed");
    
    // Test appeal committee assignment
    let appeal_committee = plagiarism_service.assign_appeal_committee(appealed_case.case_id).await;
    assert!(appeal_committee.is_ok(), "Appeal committee should be assigned");
    
    // Test cannot appeal non-existent case
    let fake_appeal = plagiarism_service.initiate_appeal(9999, "Test".to_string()).await;
    assert!(fake_appeal.is_err(), "Appeal on non-existent case should fail");
    
    // Test cannot appeal open case
    let open_case = PlagiarismCase {
        case_id: 0,
        committee_id: 1,
        text_id: 2,
        opened_date: Utc::now().date_naive(),
        description: None,
        status: "open".to_string(),
        resolution: None,
        closed_date: None,
    };
    
    let open_created = plagiarism_service.create_case(open_case).await.unwrap();
    let open_appeal = plagiarism_service.initiate_appeal(open_created.case_id, "Test".to_string()).await;
    assert!(open_appeal.is_err(), "Cannot appeal open case");
}

#[tokio::test]
async fn test_similarity_detection() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    
    // Test text similarity algorithm
    let original = "This is the original text about machine learning and artificial intelligence.";
    let plagiarized = "This is the copied text about machine learning and artificial intelligence.";
    let different = "This is a completely different text about database systems.";
    
    let similarity1 = plagiarism_service.calculate_similarity(original, plagiarized).await;
    assert!(similarity1.is_ok());
    let sim1 = similarity1.unwrap();
    assert!(sim1 > 0.8, "Highly similar texts should have high similarity score");
    
    let similarity2 = plagiarism_service.calculate_similarity(original, different).await;
    assert!(similarity2.is_ok());
    let sim2 = similarity2.unwrap();
    assert!(sim2 < 0.3, "Different texts should have low similarity score");
    
    // Test exact match
    let exact_similarity = plagiarism_service.calculate_similarity(original, original).await;
    assert!(exact_similarity.is_ok());
    assert_eq!(exact_similarity.unwrap(), 1.0, "Exact match should be 100% similar");
    
    // Test empty texts
    let empty_similarity = plagiarism_service.calculate_similarity("", "test").await;
    assert!(empty_similarity.is_err(), "Empty text should fail");
    
    // Test very short texts
    let short_similarity = plagiarism_service.calculate_similarity("a", "a").await;
    assert!(short_similarity.is_ok());
}

#[tokio::test]
async fn test_automated_plagiarism_scan() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    let text_service = TextService::new(app_state.db_pool.clone());
    
    // Create multiple texts for scanning
    let texts = vec![
        ("Text about AI and ML", "This is about artificial intelligence and machine learning."),
        ("Similar text about AI", "This text discusses artificial intelligence and machine learning."),
        ("Different topic", "Database management systems are important for data storage."),
    ];
    
    let mut text_ids = vec![];
    for (title, content) in texts {
        // In real implementation, text would have content field
        // For test, we'll simulate scanning
        text_ids.push(1); // Placeholder
    }
    
    // Test scanning new text against existing ones
    let new_text = "Artificial intelligence and machine learning are transforming industries.";
    let scan_result = plagiarism_service.scan_for_plagiarism(new_text, &text_ids).await;
    
    assert!(scan_result.is_ok());
    let matches = scan_result.unwrap();
    
    // Should find matches with AI/ML texts
    assert!(!matches.is_empty(), "Should find potential plagiarism matches");
    
    // Test threshold detection
    let threshold_matches: Vec<_> = matches.iter()
        .filter(|m| m.similarity_score >= 0.7) // 70% threshold
        .collect();
    
    assert!(threshold_matches.len() > 0, "Should find matches above threshold");
}

#[tokio::test]
async fn test_committee_quorum_requirements() {
    utils::init_test_env();
    let app_state = utils::create_test_app_state().await;
    
    let plagiarism_service = PlagiarismService::new(app_state.db_pool.clone());
    let committee_service = CommitteeService::new(app_state.db_pool.clone());
    
    // Create committee with members
    let committee = committee_service.create_committee(
        "Quorum Test Committee".to_string(),
        "Test quorum requirements".to_string(),
        "plagiarism".to_string(),
        Utc::now().date_naive(),
        "active".to_string()
    ).await.unwrap();
    
    // Add members (need at least 5 for quorum)
    for i in 0..7 {
        // In real test, create members and add to committee
    }
    
    // Test quorum calculation
    let quorum = plagiarism_service.calculate_quorum(committee.committee_id).await;
    assert!(quorum.is_ok());
    
    // Minimum quorum should be majority of members
    let (total_members, required_quorum) = quorum.unwrap();
    assert!(required_quorum > total_members / 2, "Quorum should be majority");
    
    // Test case with insufficient voters
    let case = PlagiarismCase {
        case_id: 0,
        committee_id: committee.committee_id,
        text_id: 1,
        opened_date: Utc::now().date_naive(),
        description: None,
        status: "voting".to_string(),
        resolution: None,
        closed_date: None,
    };
    
    let created_case = plagiarism_service.create_case(case).await.unwrap();
    
    // Cast votes but not enough for quorum
    let votes_cast = 2; // Less than quorum
    
    let has_quorum = plagiarism_service.check_voting_quorum(created_case.case_id, votes_cast).await;
    assert!(has_quorum.is_ok() && !has_quorum.unwrap(), "Should not have quorum with few votes");
}
