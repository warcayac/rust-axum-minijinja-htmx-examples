use eyre::Context;
use wshared::types::TResult;


#[derive(serde::Deserialize, serde::Serialize)]
pub struct MenuItem {
    pub label: String,
    pub link: String,
    pub description: String,
}

pub struct Toc {
    pub content: Vec<MenuItem>,
}

impl Toc {
    pub async fn new() -> TResult<Self> {
        let json_data = tokio::fs::read_to_string("data/toc.json")
            .await
            .wrap_err("Failed to read TOC file.")?
        ;
        let content = serde_json::from_str(&json_data).wrap_err("Failed to parse Json data.")?;
        Ok(Self { content })
    }
}