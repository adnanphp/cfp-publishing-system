use crate::domain::enums::TextStatus;

#[derive(Debug, Clone)]
pub struct Text {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
    pub status: TextStatus,
}

impl Text {
    pub fn new(id: u32, title: String, content: String, author_id: u32) -> Self {
        Self {
            id,
            title,
            content,
            author_id,
            status: TextStatus::Draft,
        }
    }
    
    pub fn can_be_edited(&self) -> bool {
        self.status.can_be_edited()
    }
}
