use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub achievement_count: u16,
    pub achievement_log_count: u16,
    pub inventory_count: u16,
    pub inventory_unique_count: u16,
    pub inventory_unique_upgradable_count: u16,
    pub inventory_unique_golden_count: u16,
    pub inventory_unique_golden_upgradable_count: u16,
    pub item_count: u16,
    pub lucky_count: u16,
    pub subscription_bonus_count: u16,
    pub subscription_bonus_limit: u16,
    pub trade_count: u16,
    pub trade_count_today: u8,
    pub trade_limit: u8,
    pub upgradable_item_count: u16,
    pub user: User
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub discord_user_name: String,
    pub discord_global_name: String,
    pub discord_id: String,
    pub discord_avatar: String,
    pub created_date: String,
    pub is_active: bool,
    pub lore_dust: u16,
    pub lore_fragment: u16,
    pub upgrade_dust: u16,
    pub balance: u16,
    pub user_banner: Option<UserBanner>,
    pub rank: Rank,
    pub leaderboards: Vec<Leaderboard>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBanner {
    pub date: String,
    pub id: String,
    pub banner: Banner
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub id: String,
    pub name: String,
    pub title: String,
    pub image_url: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub id: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct Leaderboard {
    pub position: u16,
    pub score: u16,
    pub data: LeaderboardData,
    #[serde(rename = "type")]
    pub leaderboard_type: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardData {
    pub total: u16,
    pub total_distinct: u16
}