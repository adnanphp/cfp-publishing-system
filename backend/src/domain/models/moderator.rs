use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moderator {
    pub mod_id: Uuid,
    pub member_id: Uuid,
    pub domain: String,
    pub expertise_areas: Vec<String>,
    pub approval_rate: f64,
    pub assigned_cases_count: i32,
    pub resolved_cases_count: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Moderator {
    pub fn new(member_id: Uuid, domain: String, expertise_areas: Vec<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            mod_id: Uuid::new_v4(),
            member_id,
            domain,
            expertise_areas,
            approval_rate: 0.0,
            assigned_cases_count: 0,
            resolved_cases_count: 0,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_approval_rate(&mut self, approved: bool) {
        self.assigned_cases_count += 1;
        if approved {
            self.resolved_cases_count += 1;
        }
        
        if self.assigned_cases_count > 0 {
            self.approval_rate = self.resolved_cases_count as f64 / self.assigned_cases_count as f64 * 100.0;
        }
        self.updated_at = chrono::Utc::now();
    }

    pub fn add_expertise_area(&mut self, area: String) {
        if !self.expertise_areas.contains(&area) {
            self.expertise_areas.push(area);
            self.updated_at = chrono::Utc::now();
        }
    }

    pub fn remove_expertise_area(&mut self, area: &str) {
        self.expertise_areas.retain(|a| a != area);
        self.updated_at = chrono::Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = chrono::Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = chrono::Utc::now();
    }

    pub fn has_expertise_in(&self, area: &str) -> bool {
        self.expertise_areas.iter().any(|a| a == area)
    }
}
