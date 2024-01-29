use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetail {
    pub back_urls: Vec<String>,
    pub item: Item,
    pub normal_recycle_metadata: Option<ItemMetadata>,
    pub golden_recycle_metadata: Option<ItemMetadata>,
    pub owned_percent: f64,
    pub component_items: Option<Vec<Item>>,
    pub fusion_items: Option<Vec<Item>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub identifier: u16,
    pub name: String,
    pub description: Option<String>,
    pub genre: String,
    pub slug: String,
    pub is_counting: bool,
    pub is_craftable: bool,
    pub is_goldable: bool,
    pub is_invocable: bool,
    pub is_recyclable: bool,
    pub is_tradable: bool,
    pub is_upgradable: bool,
    pub rarity: u8,
    pub score: u16,
    pub score_golden: u16,
    pub urls: Vec<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemMetadata {
    pub craft_value: u16,
    pub recycle_value: u16
}

#[derive(Debug, Deserialize)]
pub struct Fusion {
    pub item: Item,
    pub items: Vec<Item>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryEntry {
    pub id: String,
    pub is_fusion: bool,
    pub is_fusion_component: bool,
    pub is_golden: bool,
    pub item: Item,
    pub quantity: u16,
    pub recycle_metadata: ItemMetadata,
    pub upgrade_level: u8
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RarityMetadata {
    pub id: String,
    pub rarity: u8,
    pub is_golden: bool,
    pub craft_value: u16,
    pub recycle_value: u16
}