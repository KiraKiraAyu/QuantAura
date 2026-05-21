// Integration tests for Amaryllis core
// This file serves as the entry point for integration tests

#[cfg(test)]
mod database_integration {
    use amaryllis::database::init_database;

    #[tokio::test]
    async fn test_database_initialization() {
        let db = init_database("sqlite::memory:")
            .await
            .expect("Failed to initialize test database");

        assert!(db.ping().await.is_ok());
    }
}
