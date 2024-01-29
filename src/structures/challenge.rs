pub struct ActiveChallenge {
    pub id: String,
    pub begin_date: String,
    pub end_date: String,
    pub challenge: Challenge
}

pub struct Challenge {
    pub id: String,
    pub description: String,
    pub score: u16,
    pub reward_lore_dust: Option<u16>,
    pub challenge_type: ChallengeType
}

pub enum ChallengeType {
    TradeItemOnce,
    Balance,
    Daily
}