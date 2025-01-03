#[cfg(test)]
mod db_tests {
    use super::*;
    use dotenv::dotenv;
    use jordanopotato_rust::db;
    use jordanopotato_rust::models;
    use sqlx::PgPool;
    use std::env;

    // For integration tests, you'd typically import from crate root, so adjust accordingly.
    // If you're writing tests in the same file as db.rs, you won't need the module dance.
    async fn setup_test_db() -> PgPool {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL").unwrap();
        let pool = PgPool::connect(&database_url).await.unwrap();

        sqlx::query("DROP TABLE IF EXISTS conversations CASCADE;")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DROP TABLE IF EXISTS users CASCADE;")
            .execute(&pool)
            .await
            .unwrap();

        db::run_migrations(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        dotenv().ok();
        let pool = setup_test_db().await;
        // 1) Create user
        let user_name = "testuser";
        let user_prompt = "Test Prompt";
        let user = db::create_user(&pool, user_name, user_prompt)
            .await
            .unwrap();

        assert_eq!(user.name, user_name);
        assert_eq!(user.prompt.as_deref(), Some(user_prompt));

        // 2) Get user by ID
        let user_fetched = db::get_user_by_id(&pool, user.id).await.unwrap().unwrap();
        assert_eq!(user_fetched.id, user.id);
        assert_eq!(user_fetched.name, user_name);

        // 3) Update user prompt
        let new_prompt = "New Prompt from test!";
        let updated_ok = db::update_user_prompt(&pool, user.id, new_prompt)
            .await
            .unwrap();
        assert!(updated_ok);

        let updated_user = db::get_user_by_id(&pool, user.id).await.unwrap().unwrap();
        assert_eq!(updated_user.prompt.as_deref(), Some(new_prompt));
    }

    #[tokio::test]
    async fn test_get_or_create_user() {
        dotenv().ok();
        let pool = setup_test_db().await;

        let name = "maybe_exists_user";
        let default_prompt = "Default prompt";

        // If user doesn't exist, we create it
        let user1 = db::get_or_create_user(&pool, name, default_prompt)
            .await
            .unwrap();
        assert_eq!(user1.name, name);
        // If we call it again, we get the same user
        let user2 = db::get_or_create_user(&pool, name, default_prompt)
            .await
            .unwrap();
        assert_eq!(user1.id, user2.id);
    }

    #[tokio::test]
    async fn test_conversation_summary() {
        dotenv().ok();
        let pool = setup_test_db().await;

        // create user
        let user = db::create_user(&pool, "conv_user", "Some prompt")
            .await
            .unwrap();

        // update summary
        let summary_before = db::get_conversation_summary(&pool, user.id).await.unwrap();
        assert_eq!(summary_before, None); // no summary yet

        let new_summary = "This is a test conversation summary.";
        db::update_conversation_summary(&pool, user.id, new_summary)
            .await
            .unwrap();

        let summary_after = db::get_conversation_summary(&pool, user.id).await.unwrap();
        assert_eq!(summary_after, Some(new_summary.to_string()));
    }
}
