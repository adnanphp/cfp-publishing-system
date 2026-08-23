use crate::domain::enums::MemberStatus;

#[derive(Debug, Clone)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub status: MemberStatus,
}

impl Member {
    pub fn new(id: u32, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            status: MemberStatus::Pending,
        }
    }
    
    pub fn activate(&mut self) {
        self.status = MemberStatus::Active;
    }
    
    pub fn can_login(&self) -> bool {
        self.status.can_login()
    }
}
