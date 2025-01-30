use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use validator::Validate;


pub type TSharedUsers1 = Arc<RwLock<Vec<User1>>>;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)] 
pub struct User1 {
    pub id: u8,
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub active: bool,
}

impl User1 {
    pub async fn get_sample_data() -> TSharedUsers1 {
        let json_data = tokio::fs::read_to_string("data/users_1.json")
            .await
            .unwrap()
        ;
        let users = serde_json::from_str(&json_data).unwrap();
        
        Arc::new(RwLock::new(users))
    }

    pub fn default() -> Self {
        Self {
            id: 0,
            name: " ".to_string(),
            email: "null@null.xyz".to_string(),
            active: false,
        }
    }

    pub fn update(&mut self, from: User1Update) {
        if let Some(value) = from.name { self.name = value; }
        if let Some(value) = from.email { self.email = value; }
        if let Some(value) = from.active { self.active = value; }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)] 
pub struct User1Update {
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub active: Option<bool>,
}
