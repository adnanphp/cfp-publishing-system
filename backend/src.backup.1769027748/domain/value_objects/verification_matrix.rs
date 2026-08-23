use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMatrix {
    pub matrix_id: String,
    pub verification_type: VerificationType,
    pub verification_steps: Vec<VerificationStep>,
    pub current_step: usize,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationType {
    Email,
    Phone,
    Identity,
    Address,
    Payment,
    TwoFactor,
    MemberIntroduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStep {
    pub step_id: String,
    pub step_type: StepType,
    pub is_required: bool,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub verification_code: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepType {
    EmailVerification,
    PhoneVerification,
    DocumentUpload,
    PaymentVerification,
    IntroductionVerification,
    SecurityQuestions,
    BiometricVerification,
}

impl VerificationMatrix {
    pub fn new(verification_type: VerificationType, steps: Vec<VerificationStep>) -> Self {
        let mut rng = rand::thread_rng();
        let matrix_id = format!("VM{:08X}", rng.gen::<u32>());
        
        let now = Utc::now();
        let expires_at = match verification_type {
            VerificationType::Email => now + chrono::Duration::hours(24),
            VerificationType::Phone => now + chrono::Duration::hours(2),
            VerificationType::Identity => now + chrono::Duration::days(7),
            VerificationType::Address => now + chrono::Duration::days(14),
            VerificationType::Payment => now + chrono::Duration::hours(1),
            VerificationType::TwoFactor => now + chrono::Duration::minutes(15),
            VerificationType::MemberIntroduction => now + chrono::Duration::days(30),
        };

        Self {
            matrix_id,
            verification_type,
            verification_steps: steps,
            current_step: 0,
            is_completed: false,
            created_at: now,
            expires_at,
            metadata: HashMap::new(),
        }
    }

    pub fn complete_step(&mut self, step_id: &str) -> bool {
        if let Some(step) = self.verification_steps.iter_mut().find(|s| s.step_id == step_id) {
            if !step.is_completed {
                step.is_completed = true;
                step.completed_at = Some(Utc::now());
                
                // Move to next incomplete required step
                self.update_current_step();
                
                // Check if all required steps are completed
                self.check_completion();
                return true;
            }
        }
        false
    }

    fn update_current_step(&mut self) {
        for (i, step) in self.verification_steps.iter().enumerate() {
            if step.is_required && !step.is_completed {
                self.current_step = i;
                return;
            }
        }
        // If all required steps are completed, set to last step
        self.current_step = self.verification_steps.len().saturating_sub(1);
    }

    fn check_completion(&mut self) {
        let all_required_completed = self.verification_steps.iter()
            .filter(|step| step.is_required)
            .all(|step| step.is_completed);
        
        self.is_completed = all_required_completed;
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn time_remaining(&self) -> chrono::Duration {
        self.expires_at - Utc::now()
    }

    pub fn completion_percentage(&self) -> f32 {
        let total_required = self.verification_steps.iter().filter(|s| s.is_required).count();
        if total_required == 0 {
            return 0.0;
        }
        
        let completed_required = self.verification_steps.iter()
            .filter(|s| s.is_required && s.is_completed)
            .count();
        
        (completed_required as f32 / total_required as f32) * 100.0
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    pub fn generate_email_verification() -> Self {
        let steps = vec![
            VerificationStep {
                step_id: "email_verification".to_string(),
                step_type: StepType::EmailVerification,
                is_required: true,
                is_completed: false,
                completed_at: None,
                verification_code: Some(generate_verification_code()),
                metadata: HashMap::new(),
            }
        ];
        
        Self::new(VerificationType::Email, steps)
    }

    pub fn generate_two_factor_verification() -> Self {
        let steps = vec![
            VerificationStep {
                step_id: "2fa_code".to_string(),
                step_type: StepType::SecurityQuestions,
                is_required: true,
                is_completed: false,
                completed_at: None,
                verification_code: Some(generate_numeric_code(6)),
                metadata: HashMap::new(),
            }
        ];
        
        Self::new(VerificationType::TwoFactor, steps)
    }
}

impl VerificationStep {
    pub fn new(step_id: String, step_type: StepType, is_required: bool) -> Self {
        let verification_code = match step_type {
            StepType::EmailVerification => Some(generate_verification_code()),
            StepType::PhoneVerification => Some(generate_numeric_code(6)),
            StepType::TwoFactor => Some(generate_numeric_code(6)),
            _ => None,
        };

        Self {
            step_id,
            step_type,
            is_required,
            is_completed: false,
            completed_at: None,
            verification_code,
            metadata: HashMap::new(),
        }
    }
}

fn generate_verification_code() -> String {
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

fn generate_numeric_code(length: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}
