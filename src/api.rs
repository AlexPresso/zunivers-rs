use crate::structures::pack::Pack;
use crate::structures::card::Item;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;


const BASE_URL: &str = "https://zunivers-api.zerator.com";
const PACK: &str = "/public/pack";
const ITEMS: &str = "/public/item";
const INVENTORY: &str = "/public/inventory";


pub async fn fetch_packs() -> Result<Vec<Pack>, Error> {
    request(String::from(PACK)).await
}

pub async fn fetch_items() -> Result<Vec<Item>, Error> {
    request(String::from(ITEMS)).await
}

pub async fn fetch_item(slug: String) -> Result<Item, Error> {
    request(format!("{}?item={}", String::from(ITEMS), slug)).await
}

pub async fn fetch_inventory(user: String) -> Result<Vec<Item>, Error> {
    request(format!("{}/{}", INVENTORY, user)).await
}

async fn request<T>(uri: String) -> Result<T, Error>
where
    T: DeserializeOwned
{
    let result = Client::new()
        .get(format!("{}{}", BASE_URL, uri))
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(result)
}
