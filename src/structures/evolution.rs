use serde::Deserialize;
use crate::structures::card::Item;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evolution {
    #[serde(rename = "evolutionItems")]
    pub items: Vec<EvolutionItem>,
    #[serde(rename = "evolutionUpgradeDustCost")]
    pub costs: Vec<u16>,
    pub upgrade_dust: u16
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionItem {
    pub is_owned: bool,
    pub item: Item
}