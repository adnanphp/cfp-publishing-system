use crate::domain::enums::CharityStatus;

#[derive(Debug, Clone)]
pub struct Charity {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub status: CharityStatus,
    pub verified: bool,
}

impl Charity {
    pub fn new(id: u32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            status: CharityStatus::Pending,
            verified: false,
        }
    }
    
    pub fn verify(&mut self) {
        self.verified = true;
        self.status = CharityStatus::Active;
    }
    
    pub fn can_receive_donations(&self) -> bool {
        self.verified && self.status.can_receive_donations()
    }
}
