use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::model::card::Item;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveChallenge {
    pub id: String,
    pub begin_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub challenge: Challenge,
    pub progress: Option<ChallengeProgress>,
    pub challenge_log: Option<ChallengeLog>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: String,
    pub description: String,
    pub score: u16,
    pub reward_lore_dust: u16,
    #[serde(rename = "type")]
    pub challenge_type: String
}

#[derive(Debug, Deserialize)]
pub struct ChallengeProgress {
    pub current: u16,
    pub max: u16,
    #[serde(rename = "type")]
    pub progress_type: String,
    pub items: Option<Vec<Item>>
}

#[derive(Debug, Deserialize)]
pub struct ChallengeLog {

}
