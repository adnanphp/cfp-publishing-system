// This module contains raw SQL query strings for complex queries
// that might be reused across different repositories

pub mod member_queries;
pub mod text_queries;
pub mod donation_queries;
pub mod plagiarism_queries;

// Re-export for convenience
pub use member_queries::*;
pub use text_queries::*;
pub use donation_queries::*;
pub use plagiarism_queries::*;

// Common query fragments
pub mod fragments {
    // Pagination fragment
    pub const PAGINATION: &str = "LIMIT :limit OFFSET :offset";
    
    // Order by fragment
    pub fn order_by(field: &str, order: &str) -> String {
        format!("ORDER BY {} {}", field, order)
    }
    
    // Where clause builder
    pub fn build_where_clause(conditions: &[(&str, &str)]) -> String {
        if conditions.is_empty() {
            return String::new();
        }
        
        let clauses: Vec<String> = conditions
            .iter()
            .map(|(field, _)| format!("{} = :{}", field, field))
            .collect();
        
        format!("WHERE {}", clauses.join(" AND "))
    }
    
    // Search fragment
    pub fn search_clause(fields: &[&str]) -> String {
        let search_conditions: Vec<String> = fields
            .iter()
            .map(|field| format!("{} ILIKE :search_term", field))
            .collect();
        
        if search_conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", search_conditions.join(" OR "))
        }
    }
}
