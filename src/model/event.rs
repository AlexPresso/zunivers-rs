use serde::Deserialize;
use crate::model::card::Item;
use crate::model::pack::Pack;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub begin_date: String,
    pub end_date: String,
    pub balance_cost: u16,
    pub lore_dust_cost: u16,
    pub invoke_count: u16,
    pub is_active: bool,
    pub is_one_time: bool,
    pub is_pity_enabled: bool,
    pub pack: Pack,
    pub items: Vec<Item>
}