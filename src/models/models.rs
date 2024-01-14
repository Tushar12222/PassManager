use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PasswordDetails {
    pub website: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Response {
    pub success: bool,
    pub message: Option<Vec<PasswordDetails>>
}
