use serde::Deserialize;
use crate::structures::pack::Pack;

#[derive(Debug, Deserialize)]
pub struct Card {
    pub back_urls: Vec<String>,
    pub item: Item,
    pub pack: Pack
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub genre: String,
    pub slug: String,
    pub identifier: u16,
    pub is_counting: bool,
    pub is_craftable: bool,
    pub is_goldable: bool,
    pub is_invocable: bool,
    pub is_recyclable: bool,
    pub is_tradable: bool,
    pub is_upgradable: bool,
    pub rarity: u8,
    pub score: u16,
    pub score_golden: u16
}