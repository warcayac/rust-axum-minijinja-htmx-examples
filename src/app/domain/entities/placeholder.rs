use std::sync::Arc;

use tokio::sync::RwLock;


pub type TSharedPlaceholders = Arc<RwLock<Vec<Placeholder>>>;

#[derive(Debug, Clone, serde::Deserialize)] 
pub struct Placeholder {
    pub content: String,
}

impl Placeholder {
    pub async fn get_sample_data() -> TSharedPlaceholders {
        let json_data = tokio::fs::read_to_string("data/placeholders.json")
            .await
            .unwrap()
        ;
        let data = serde_json::from_str(&json_data).unwrap();
        
        Arc::new(RwLock::new(data))
    }
}