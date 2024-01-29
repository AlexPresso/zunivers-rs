use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveChallenge {
    pub id: String,
    pub begin_date: String,
    pub end_date: String,
    pub challenge: Challenge
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
