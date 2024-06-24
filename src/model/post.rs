use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub created_by: String,
    pub date: NaiveDateTime,
    pub slug: String,
    pub image_url: String
}