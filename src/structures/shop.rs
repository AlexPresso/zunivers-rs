use serde::Deserialize;
use crate::structures::card::Item;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopEntry {
    pub id: String,
    pub initial_stock: u8,
    pub current_stock: u8,
    pub shop_inventory: ShopInventory
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopInventory {
    pub item: Item,
    pub balance: u16,
    pub is_golden: bool,
    pub upgrade_level: u8
}