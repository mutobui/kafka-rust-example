use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserAction {
    pub user_id: u32,
    pub action: String,
    pub producer_id: String,
}