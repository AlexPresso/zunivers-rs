use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub created_by: String,
    pub date: DateTime<Utc>,
    pub slug: String,
    pub image_url: String
}