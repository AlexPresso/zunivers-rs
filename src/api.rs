use std::fmt::{Debug, Display};
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;

use crate::model::app::{AppSettings, AppStatus};
use crate::model::card::{Fusion, InventoryEntry, Item, ItemDetail, RarityMetadata};
use crate::model::challenge::ActiveChallenge;
use crate::model::event::Event;
use crate::model::pack::Pack;
use crate::model::post::Post;
use crate::model::rayou::Jackpot;
use crate::model::shop::ShopEntry;
use crate::model::user::{Profile, UserBanner, UserLoot};
use crate::model::vortex::{Tournament, VortexSeason};


const BASE_URL: &str = "https://zunivers-api.zerator.com";


macro_rules! define_endpoint {
    (st = $structure:ty, url = $url:literal) => {
        define_endpoint!(st = $structure, url = $url, slug = false, fetch => Self, fetch_with_params => Self);
    };
    (st = $structure:ty, url = $url:literal, slug = true) => {
        define_endpoint!(st = $structure, url = $url, slug = true, fetch => Self, fetch_with_params => Self);
    };

    (
        st      =   $structure:ty,
        url     =   $url:literal,
        slug    =   false,
        $fn_name:ident => $return_type:ty,
        $fn_p_name:ident => $return_p_type:ty
    ) => {
        impl $structure {
            pub async fn $fn_name() -> Result<$return_type, Error> {
                request($url, &[]).await
            }

            pub async fn $fn_p_name(params: &[(String, String)]) -> Result<$return_p_type, Error> {
                request($url, params).await
            }
        }
    };

    (
        st      =   $structure:ty,
        url     =   $url:literal,
        slug    =   true,
        $fn_name:ident => $return_type:ty,
        $fn_p_name:ident => $return_p_type:ty
    ) => {
        impl $structure {
            pub async fn $fn_name<S>(slug: S) -> Result<$return_type, Error>
                where S: Into<String> + Display
            {
                request(format!("{}/{}", $url, slug), &[]).await
            }

            pub async fn $fn_p_name<S>(slug: S, params: &[(String, String)]) -> Result<$return_p_type, Error>
                where S: Into<String> + Display
            {
                request(format!("{}/{}", $url, slug), params).await
            }
        }
    }
}


define_endpoint!(st = Item, url = "/public/item", slug = false, fetch_all => Vec<Self>, fetch_all_params => Vec<Self>);
define_endpoint!(st = ItemDetail, url = "/public/item", slug = true);
define_endpoint!(st = Pack, url = "/public/pack", slug = false, fetch_all => Vec<Self>, fetch_all_params => Vec<Self>);
define_endpoint!(st = Fusion, url = "/public/fusion", slug = false, fetch_all => Vec<Self>, fetch_all_params => Vec<Self>);
define_endpoint!(st = RarityMetadata, url = "/public/recycle/config", slug = false, fetch_all => Vec<Self>, fetch_all_params => Vec<Self>);

define_endpoint!(st = Profile, url = "/public/user", slug = true);
define_endpoint!(st = UserLoot, url = "/public/loot", slug = true, fetch_for => Self, fetch_for_params => Self);
define_endpoint!(st = InventoryEntry, url = "/public/inventory", slug = true, fetch_for => Vec<Self>, fetch_for_params => Vec<Self>);
define_endpoint!(st = UserBanner, url = "/public/banner", slug = true, fetch_for => Vec<Self>, fetch_for_params => Vec<Self>);

define_endpoint!(st = AppStatus, url = "/app/status");
define_endpoint!(st = AppSettings, url = "/app/settings");

define_endpoint!(st = Post, url = "/public/post", slug = true);
define_endpoint!(st = Post, url = "/public/post", slug = false, fetch_all => Vec<Self>, fetch_all_params => Vec<Self>);

define_endpoint!(st = Jackpot, url = "/public/lucky/jackpot");
define_endpoint!(st = ShopEntry, url = "/public/shop", slug = false, fetch_all => Vec<Self>, fetch_all_params => Vec<Self>);
define_endpoint!(st = Event, url = "/public/event/current", slug = false, fetch_current => Vec<Self>, fetch_current_params => Vec<Self>);
define_endpoint!(st = ActiveChallenge, url = "/public/challenge", slug = false, fetch_all => Vec<Self>, fetch_all_params => Vec<Self>);
define_endpoint!(st = ActiveChallenge, url = "/public/challenge", slug = true, fetch_for => Vec<Self>, fetch_for_params => Vec<Self>);
define_endpoint!(st = VortexSeason, url = "/public/tower/season");
define_endpoint!(st = Tournament, url = "/public/tournament/latest");


async fn request<T, S>(uri: S, params: &[(String, String)]) -> Result<T, Error>
    where
        T: DeserializeOwned + Debug,
        S: Into<String> + Display
{
    Client::new()
        .get(format!("{}{}", BASE_URL, uri))
        .query(params)
        .send()
        .await?
        .json::<T>()
        .await
}