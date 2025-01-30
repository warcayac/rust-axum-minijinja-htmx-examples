use std::sync::Arc;

use tokio::sync::RwLock;


pub type TSharedCarBrand = Arc<RwLock<Vec<CarBrand>>>;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CarBrand {
    pub name: String,
    pub models: Vec<String>,
}

impl CarBrand {
    pub async fn get_sample_data() -> TSharedCarBrand {
        let json_data = tokio::fs::read_to_string("data/car_brands.json")
            .await
            .unwrap()
        ;
        let data = serde_json::from_str(&json_data).unwrap();
        
        Arc::new(RwLock::new(data))
    }
}