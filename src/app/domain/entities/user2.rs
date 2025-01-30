use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use validator::Validate;


pub type TSharedUsers2 = Arc<RwLock<Vec<User2>>>;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)] 
pub struct User2 {
    pub id: u8,
    #[validate(length(min = 1, max = 50))]
    pub first_name: String,
    #[validate(length(min = 1, max = 50))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
}

impl User2 {
    pub async fn get_sample_data() -> TSharedUsers2 {
        let json_data = tokio::fs::read_to_string("data/users_2.json")
            .await
            .unwrap()
        ;
        let users = serde_json::from_str(&json_data).unwrap();
        
        Arc::new(RwLock::new(users))
    }

    // pub fn update(&mut self, from: User2Update) {
    //     if let Some(value) = from.first_name { self.first_name = value; }
    //     if let Some(value) = from.last_name { self.last_name = value; }
    //     if let Some(value) = from.email { self.email = value; }
    // }

    // pub fn default() -> Self {
    //     Self {
    //         id: 0,
    //         first_name: " ".to_string(),
    //         last_name: " ".to_string(),
    //         email: "null@null.xyz".to_string(),
    //     }
    // }
}

#[derive(Debug, Deserialize, Validate)] 
pub struct User2Update {
    #[validate(length(min = 1, max = 50))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}
