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
        let item = fetch_item(String::from("4030-adrien-nou-garros")).await;

        assert!(item.is_ok());
    }

    #[tokio::test]
    async fn test_inventory() {
        let inventory = fetch_inventory(String::from("alexpresso")).await;

        assert!(inventory.is_ok());
        assert!(inventory.unwrap().len() > 0)
    }
}