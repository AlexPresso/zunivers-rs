use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStatus {
    pub application_version: String,
    pub db_version: String,
    pub instance_id: String,
    pub uptime: u64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub current_event_id: Option<String>,
    pub in_calendar_period: bool
}
