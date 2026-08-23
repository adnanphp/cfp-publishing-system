use crate::{
    domain::enums::VoteType,
    utils::error::AppError,
};

pub struct VoteValidator;

impl VoteValidator {
    pub fn validate_vote_type(vote: &VoteType) -> Result<(), AppError> {
        // All VoteType variants are valid
        match vote {
            VoteType::Plagiarized | VoteType::NotPlagiarized | VoteType::Abstain => Ok(())
        }
    }
    
    pub fn validate_rationale(rationale: &Option<String>) -> Result<(), AppError> {
        if let Some(rationale) = rationale {
            // Rationale must be between 10 and 1000 characters if provided
            if rationale.len() < 10 {
                return Err(AppError::validation_error(
                    "Rationale must be at least 10 characters if provided"
                ));
            }
            
            if rationale.len() > 1000 {
                return Err(AppError::validation_error(
                    "Rationale cannot exceed 1000 characters"
                ));
            }
            
            // Rationale should have meaningful content
            let word_count = rationale.split_whitespace().count();
            if word_count < 3 {
                return Err(AppError::validation_error(
                    "Rationale must contain at least 3 words"
                ));
            }
        }
        
        Ok(())
    }
    
    pub fn validate_confidence_level(confidence_level: &Option<i32>) -> Result<(), AppError> {
        if let Some(level) = confidence_level {
            // Confidence level must be between 1 and 10
            if *level < 1 || *level > 10 {
                return Err(AppError::validation_error(
                    "Confidence level must be between 1 and 10"
                ));
            }
        }
        
        Ok(())
    }
    
    pub fn validate_has_downloaded_text(has_downloaded: bool) -> Result<(), AppError> {
        // Members must have downloaded the text to vote
        if !has_downloaded {
            return Err(AppError::validation_error(
                "You must download the text before voting"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_voting_period(is_voting_open: bool, opened_date: chrono::DateTime<chrono::Utc>) -> Result<(), AppError> {
        if !is_voting_open {
            return Err(AppError::validation_error(
                "Voting is not currently open for this case"
            ));
        }
        
        // Voting period is 14 days
        let voting_end = opened_date + chrono::Duration::days(14);
        let now = chrono::Utc::now();
        
        if now > voting_end {
            return Err(AppError::validation_error(
                "Voting period has ended for this case"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_has_not_voted(has_voted: bool, allow_vote_change: bool) -> Result<(), AppError> {
        if has_voted && !allow_vote_change {
            return Err(AppError::validation_error(
                "You have already voted on this case and vote changes are not allowed"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_vote_quorum(total_votes: i32, required_votes: i32, min_quorum_percentage: i32) -> Result<(), AppError> {
        if total_votes == 0 {
            return Ok(()); // No votes yet, quorum not reached
        }
        
        let quorum_percentage = (total_votes as f64 / required_votes as f64) * 100.0;
        
        if quorum_percentage < min_quorum_percentage as f64 {
            return Err(AppError::validation_error(
                format!("Quorum not reached. Current: {:.1}%, Required: {}%", 
                    quorum_percentage, min_quorum_percentage)
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_two_thirds_majority(plagiarized_votes: i32, total_votes_cast: i32) -> Result<(), AppError> {
        if total_votes_cast == 0 {
            return Ok(()); // No votes yet
        }
        
        let required_majority = (total_votes_cast as f64 * 2.0 / 3.0).ceil() as i32;
        
        if plagiarized_votes < required_majority {
            return Err(AppError::validation_error(
                format!("Two-thirds majority not reached. Plagiarized votes: {}, Required: {}", 
                    plagiarized_votes, required_majority)
            ));
        }
        
        Ok(())
    }
}
