#[derive(Debug, serde::Deserialize)]
pub struct Payload {
    #[serde(rename = "note-text")]
    pub note: String,
}