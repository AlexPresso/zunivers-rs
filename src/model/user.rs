use std::collections::HashMap;
use serde::Deserialize;
use chrono::{NaiveDate, NaiveDateTime};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub achievement_count: u16,
    pub achievement_log_count: u16,
    pub inventory_count: u32,
    pub inventory_unique_count: u32,
    pub inventory_unique_upgradable_count: u32,
    pub inventory_unique_golden_count: u32,
    pub inventory_unique_golden_upgradable_count: u32,
    pub item_count: u128,
    pub lucky_count: u128,
    pub subscription_bonus_count: u32,
    pub subscription_bonus_limit: u32,
    pub trade_count: u32,
    pub trade_count_today: u16,
    pub trade_limit: u8,
    pub upgradable_item_count: u16,
    pub user: User,
    pub leaderboards: Vec<Leaderboard>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub discord_user_name: String,
    pub discord_global_name: Option<String>,
    pub discord_id: String,
    pub discord_avatar: String,
    pub created_date: NaiveDateTime,
    pub is_active: bool,
    pub lore_dust: u128,
    pub lore_fragment: u128,
    pub upgrade_dust: u128,
    pub balance: u128,
    pub user_banner: Option<UserBanner>,
    pub rank: Option<Rank>
}

#[derive(Debug, Deserialize)]
pub struct UserBanner {
    pub id: Option<String>,
    pub date: Option<NaiveDateTime>,
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
pub struct Rank {
    pub id: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct Leaderboard {
    pub position: u32,
    pub score: u128,
    pub data: Option<HashMap<String, u32>>,
    #[serde(rename = "type")]
    pub leaderboard_type: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoot {
    pub loot_infos: Vec<LootInfo>
}

#[derive(Debug, Deserialize)]
pub struct LootInfo {
    pub count: u32,
    pub date: NaiveDate
}

