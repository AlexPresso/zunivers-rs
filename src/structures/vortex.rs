use serde::Deserialize;
use crate::structures::pack::Pack;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VortexSeason {
    pub id: String,
    pub index: u16,
    pub begin_date: String,
    pub end_date: String,
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
    pub rate: u16,
    pub drop_rates: Vec<DropRate>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRate {
    pub id: String,
    pub is_golden: bool,
    pub rarity: u16,
    pub rate: u16
}