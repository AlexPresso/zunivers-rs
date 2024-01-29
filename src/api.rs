use std::fmt::Debug;
use crate::structures::pack::Pack;
use crate::structures::card::{Fusion, InventoryEntry, Item, ItemDetail, RarityMetadata};
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use crate::structures::app::{AppSettings, AppStatus};
use crate::structures::challenge::ActiveChallenge;
use crate::structures::event::Event;
use crate::structures::post::Post;
use crate::structures::rayou::Jackpot;
use crate::structures::shop::ShopEntry;
use crate::structures::user::Profile;
use crate::structures::vortex::{Tournament, VortexSeason};


const BASE_URL: &str = "https://zunivers-api.zerator.com";
const PACK: &str = "/public/pack";
const ITEMS: &str = "/public/item";
const FUSION: &str = "/public/fusion";
const INVENTORY: &str = "/public/inventory";
const USER: &str = "/public/user";
const POSTS: &str = "/public/post";
const LUCKY: &str = "/public/lucky/jackpot";
const SHOP: &str = "/public/shop";
const EVENT: &str = "/public/event/current";
const TOWER: &str = "/public/tower/season";
const TOURNAMENT: &str = "/public/tournament/latest";
const CHALLENGE: &str = "/public/challenge";
const RECYCLE: &str = "/public/recycle/config";
const STATUS: &str = "/app/status";
const SETTINGS: &str = "/app/settings";


pub async fn fetch_packs() -> Result<Vec<Pack>, Error> {
    request(String::from(PACK)).await
}

pub async fn fetch_items() -> Result<Vec<Item>, Error> {
    request(String::from(ITEMS)).await
}

pub async fn fetch_item(slug: &String) -> Result<ItemDetail, Error> {
    request(format!("{}/{}", ITEMS, slug)).await
}

pub async fn fetch_fusions() -> Result<Vec<Fusion>, Error> {
    request(String::from(FUSION)).await
}

pub async fn fetch_inventory(username: &String) -> Result<Vec<InventoryEntry>, Error> {
    request(format!("{}/{}", INVENTORY, username)).await
}

pub async fn fetch_user_profile(username: &String) -> Result<Profile, Error> {
    request(format!("{}/{}", USER, username)).await
}

pub async fn fetch_app_status() -> Result<AppStatus, Error> {
    request(String::from(STATUS)).await
}

pub async fn fetch_app_settings() -> Result<AppSettings, Error> {
    request(String::from(SETTINGS)).await
}

pub async fn fetch_posts() -> Result<Vec<Post>, Error> {
    request(String::from(POSTS)).await
}

pub async fn fetch_post(slug: &String) -> Result<Post, Error> {
    request(format!("{}/{}", POSTS, slug)).await
}

pub async fn fetch_jackpot() -> Result<Jackpot, Error> {
    request(String::from(LUCKY)).await
}

pub async fn fetch_shop() -> Result<Vec<ShopEntry>, Error> {
    request(String::from(SHOP)).await
}

pub async fn fetch_current_events() -> Result<Option<Vec<Event>>, Error> {
    request(String::from(EVENT)).await
}

pub async fn fetch_active_challenges() -> Result<Vec<ActiveChallenge>, Error> {
    request(String::from(CHALLENGE)).await
}

pub async fn fetch_vortex_season() -> Result<VortexSeason, Error> {
    request(String::from(TOWER)).await
}

pub async fn fetch_vortex_tournament() -> Result<Tournament, Error> {
    request(String::from(TOURNAMENT)).await
}

pub async fn fetch_recycle_config() -> Result<Vec<RarityMetadata>, Error> {
    request(String::from(RECYCLE)).await
}


async fn request<T>(uri: String) -> Result<T, Error>
where
    T: DeserializeOwned + Debug
{
    let result = Client::new()
        .get(format!("{}{}", BASE_URL, uri))
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(result)
}
