use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}
