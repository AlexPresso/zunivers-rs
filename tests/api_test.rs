#[cfg(test)]
mod test {
    use zunivers_rs::structures::app::{AppSettings, AppStatus};
    use zunivers_rs::structures::card::{Fusion, InventoryEntry, Item, ItemDetail, RarityMetadata};
    use zunivers_rs::structures::challenge::ActiveChallenge;
    use zunivers_rs::structures::event::Event;
    use zunivers_rs::structures::pack::Pack;
    use zunivers_rs::structures::post::Post;
    use zunivers_rs::structures::rayou::Jackpot;
    use zunivers_rs::structures::shop::ShopEntry;
    use zunivers_rs::structures::user::Profile;
    use zunivers_rs::structures::vortex::{Tournament, VortexSeason};

    #[tokio::test]
    async fn test_packs() {
        let packs = Pack::fetch_all().await;

        assert!(packs.is_ok());
        assert!(packs.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_items() {
        let items = Item::fetch_all().await;

        assert!(items.is_ok());
        assert!(items.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_search_items() {
        let items = Item::fetch_all_params(&[
            ("search".to_string(), "Garros".to_string())
        ]).await;

        assert!(items.is_ok());
        assert_eq!(items.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_item() {
        let item = ItemDetail::fetch(&String::from("4030-adrien-nou-garros")).await;
        assert!(item.is_ok());
    }

    #[tokio::test]
    async fn test_fusion() {
        let fusions = Fusion::fetch_all().await;

        assert!(fusions.is_ok());
        assert!(fusions.unwrap().len() > 0)
    }

    #[tokio::test]
    async fn test_inventory() {
        let inventory = InventoryEntry::fetch_for(&String::from("alexpresso")).await;

        assert!(inventory.is_ok());
        assert!(inventory.unwrap().len() > 0)
    }

    #[tokio::test]
    async fn test_user_profile() {
        let user = Profile::fetch(&String::from("alexpresso")).await;
        assert!(user.is_ok());
    }

    #[tokio::test]
    async fn test_app_status() {
        let status = AppStatus::fetch().await;
        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn test_app_settings() {
        let settings = AppSettings::fetch().await;
        assert!(settings.is_ok());
    }

    #[tokio::test]
    async fn test_posts() {
        let posts = Post::fetch_all().await;

        assert!(posts.is_ok());
        assert!(posts.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_post() {
        let post = Post::fetch(&String::from("patch-notes-2-1")).await;

        assert!(post.is_ok());
        assert!(post.unwrap().content.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_jackpot() {
        let jackpot = Jackpot::fetch().await;
        assert!(jackpot.is_ok());
    }

    #[tokio::test]
    async fn test_shop() {
        let shop = ShopEntry::fetch_all().await;

        assert!(shop.is_ok());
        assert!(shop.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_current_events() {
        let events = Event::fetch_current().await;

        assert!(events.is_ok());
    }

    #[tokio::test]
    async fn test_active_challenges() {
        let challenges = ActiveChallenge::fetch_all().await;

        assert!(challenges.is_ok());
        assert!(challenges.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_vortex_season() {
        let vortex = VortexSeason::fetch().await;

        assert!(vortex.is_ok());
    }

    #[tokio::test]
    async fn test_vortex_tournament() {
        let tournament = Tournament::fetch().await;

        assert!(tournament.is_ok());
    }

    #[tokio::test]
    async fn test_recycle_config() {
        let config = RarityMetadata::fetch_all().await;

        assert!(config.is_ok());
    }
}