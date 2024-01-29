#[cfg(test)]
mod test {
    use zunivers_rs::api::*;

    #[tokio::test]
    async fn test_packs() {
        let packs = fetch_packs().await;

        assert!(packs.is_ok());
        assert!(packs.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_items() {
        let items = fetch_items().await;

        assert!(items.is_ok());
        assert!(items.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_item() {
        let item = fetch_item(&String::from("4030-adrien-nou-garros")).await;
        assert!(item.is_ok());
    }

    #[tokio::test]
    async fn test_fusion() {
        let fusions = fetch_fusions().await;

        assert!(fusions.is_ok());
        assert!(fusions.unwrap().len() > 0)
    }

    #[tokio::test]
    async fn test_inventory() {
        let inventory = fetch_inventory(&String::from("alexpresso")).await;

        assert!(inventory.is_ok());
        assert!(inventory.unwrap().len() > 0)
    }

    #[tokio::test]
    async fn test_user_profile() {
        let user = fetch_user_profile(&String::from("alexpresso")).await;
        assert!(user.is_ok());
    }

    #[tokio::test]
    async fn test_app_status() {
        let status = fetch_app_status().await;
        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn test_app_settings() {
        let settings = fetch_app_settings().await;
        assert!(settings.is_ok());
    }

    #[tokio::test]
    async fn test_posts() {
        let posts = fetch_posts().await;

        assert!(posts.is_ok());
        assert!(posts.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_post() {
        let post = fetch_post(&String::from("patch-notes-2-1")).await;

        assert!(post.is_ok());
        assert!(post.unwrap().content.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_jackpot() {
        let jackpot = fetch_jackpot().await;
        assert!(jackpot.is_ok());
    }

    #[tokio::test]
    async fn test_shop() {
        let shop = fetch_shop().await;

        assert!(shop.is_ok());
        assert!(shop.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_current_events() {
        let events = fetch_current_events().await;

        assert!(events.is_ok());
        assert!(events.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_active_challenges() {
        let challenges = fetch_active_challenges().await;

        assert!(challenges.is_ok());
        assert!(challenges.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_vortex_season() {
        let vortex = fetch_vortex_season().await;

        assert!(vortex.is_ok());
    }
}