use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use validator::Validate;


pub type TSharedContact = Arc<RwLock<Contact>>;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Contact {
    #[validate(length(min = 1, max = 30, message = "First name must be between 1 and 30 characters"))]
    pub first_name: String,
    #[validate(length(min = 1, max = 30, message = "Last name must be between 1 and 30 characters"))]
    pub last_name: String,
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
}

impl Contact {
    pub fn get_sample_data() -> TSharedContact {
        let value = Self {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        };
        Arc::new(RwLock::new(value))
    }
}