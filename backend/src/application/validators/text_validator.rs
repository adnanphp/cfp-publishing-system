use crate::utils::error::AppError;

pub struct TextValidator;

impl TextValidator {
    pub fn validate_title(title: &str) -> Result<(), AppError> {
        if title.len() < 5 || title.len() > 255 {
            return Err(AppError::validation_error(
                "Title must be between 5 and 255 characters"
            ));
        }
        
        // Title cannot consist only of numbers or special characters
        let alphabetic_count = title.chars().filter(|c| c.is_alphabetic()).count();
        if alphabetic_count < 3 {
            return Err(AppError::validation_error(
                "Title must contain at least 3 letters"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_abstract(abstract_text: &str) -> Result<(), AppError> {
        if abstract_text.len() < 50 || abstract_text.len() > 2000 {
            return Err(AppError::validation_error(
                "Abstract must be between 50 and 2000 characters"
            ));
        }
        
        // Abstract should have meaningful content
        let word_count = abstract_text.split_whitespace().count();
        if word_count < 10 {
            return Err(AppError::validation_error(
                "Abstract must contain at least 10 words"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_topic(topic: &str) -> Result<(), AppError> {
        if topic.len() < 2 || topic.len() > 100 {
            return Err(AppError::validation_error(
                "Topic must be between 2 and 100 characters"
            ));
        }
        
        // Topic should not contain special characters except hyphen and space
        if topic.chars().any(|c| {
            !c.is_alphanumeric() && !c.is_whitespace() && c != '-'
        }) {
            return Err(AppError::validation_error(
                "Topic contains invalid characters. Only letters, numbers, spaces, and hyphens are allowed."
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_keywords(keywords: &[String]) -> Result<(), AppError> {
        // Maximum 10 keywords
        if keywords.len() > 10 {
            return Err(AppError::validation_error(
                "Maximum 10 keywords allowed"
            ));
        }
        
        // Each keyword must be between 2 and 50 characters
        for keyword in keywords {
            if keyword.len() < 2 || keyword.len() > 50 {
                return Err(AppError::validation_error(
                    "Each keyword must be between 2 and 50 characters"
                ));
            }
            
            // Keywords should not contain special characters except hyphen
            if keyword.chars().any(|c| {
                !c.is_alphanumeric() && c != '-'
            }) {
                return Err(AppError::validation_error(
                    format!("Keyword '{}' contains invalid characters. Only letters, numbers, and hyphens are allowed.", keyword)
                ));
            }
        }
        
        // No duplicates (case-insensitive)
        let mut seen = std::collections::HashSet::new();
        for keyword in keywords {
            let lower_keyword = keyword.to_lowercase();
            if seen.contains(&lower_keyword) {
                return Err(AppError::validation_error(
                    format!("Duplicate keyword: {}", keyword)
                ));
            }
            seen.insert(lower_keyword);
        }
        
        Ok(())
    }
    
    pub fn validate_content(content: &str) -> Result<(), AppError> {
        if content.len() < 100 {
            return Err(AppError::validation_error(
                "Content must be at least 100 characters"
            ));
        }
        
        // Maximum content size: 10MB
        if content.len() > 10 * 1024 * 1024 {
            return Err(AppError::validation_error(
                "Content exceeds maximum size of 10MB"
            ));
        }
        
        // Check for minimum word count
        let word_count = content.split_whitespace().count();
        if word_count < 50 {
            return Err(AppError::validation_error(
                "Content must contain at least 50 words"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_version_number(version: i32) -> Result<(), AppError> {
        if version < 1 {
            return Err(AppError::validation_error(
                "Version number must be at least 1"
            ));
        }
        
        if version > 1000 {
            return Err(AppError::validation_error(
                "Version number cannot exceed 1000"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_changes(changes: &str) -> Result<(), AppError> {
        if changes.len() < 10 {
            return Err(AppError::validation_error(
                "Change description must be at least 10 characters"
            ));
        }
        
        if changes.len() > 10000 {
            return Err(AppError::validation_error(
                "Change description cannot exceed 10,000 characters"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_change_summary(summary: &str) -> Result<(), AppError> {
        if summary.len() < 10 || summary.len() > 500 {
            return Err(AppError::validation_error(
                "Change summary must be between 10 and 500 characters"
            ));
        }
        
        Ok(())
    }
}
