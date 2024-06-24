use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;

use crate::model::pack::Pack;
use crate::model::user::User;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VortexSeason {
    pub id: String,
    pub index: u16,
    pub begin_date: NaiveDate,
    pub end_date: NaiveDate,
    pub tower: Tower
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tower {
    pub id: String,
    pub name: String,
    pub index: u16,
    pub image_url: String,
    pub pack: Pack,
    pub reputation: Reputation,
    pub tower_floors: Vec<TowerFloor>
}

#[derive(Debug, Deserialize)]
pub struct Reputation {
    pub id: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerFloor {
    pub id: String,
    pub index: u16,
    pub rate: f32,
    pub drop_rates: Vec<DropRate>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRate {
    pub id: String,
    pub is_golden: bool,
    pub rarity: u16,
    pub rate: f32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub id: String,
    pub is_done: bool,
    pub date: NaiveDateTime,
    pub next_round_date: Option<NaiveDateTime>,
    pub round_count: u16,
    pub round_index: u16,
    pub battles: Vec<Battle>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battle {
    pub id: String,
    pub index: u16,
    pub round_index: u16,
    pub outcome: Option<bool>,
    pub participant1: BattlePlayer,
    pub participant2: Option<BattlePlayer>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BattlePlayer {
    pub id: String,
    pub bonus: f32,
    pub user: User
}