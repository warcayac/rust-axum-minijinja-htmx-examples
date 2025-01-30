use std::sync::Arc;

use wshared::types::TResult;
use super::{domain::entities::*, init_templates, TSharedTemplates};


pub type TSharedAppState = Arc<AppState>;
pub type TAppStateRouter = axum::Router<TSharedAppState>;
pub type TAppStateState = axum::extract::State<TSharedAppState>;


#[derive(Clone)]
pub struct AppState {
    pub env: TSharedTemplates,
    pub contact: TSharedContact,
    pub users1: TSharedUsers1,
    pub users2: TSharedUsers2,
    pub car_brands: TSharedCarBrand,
    pub placeholders: TSharedPlaceholders,
}

impl AppState {
    pub async fn new() -> TResult<TSharedAppState> {
        let value = Self {
            env: init_templates()?,
            contact: Contact::get_sample_data(),
            users1: User1::get_sample_data().await,
            users2: User2::get_sample_data().await,
            car_brands: CarBrand::get_sample_data().await,
            placeholders: Placeholder::get_sample_data().await,
        };
        Ok(Arc::new(value))
    }
}
