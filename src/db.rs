use anyhow::Result;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

use crate::models::{Conversation, User};

/// Initialize a connection pool for the Postgres DB.
/// This function explicitly calls dotenv().ok() to load env vars from a .env file.
/// Typically, you might call dotenv().ok() in main.rs instead for the entire application.
pub async fn init_db_pool() -> Result<Pool<Postgres>> {
    // Loads environment variables from a .env file
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env or environment variables");

    // Create a connection pool with some default settings:
    let pool = PgPoolOptions::new()
        .max_connections(5) // tune as needed
        .connect(&database_url)
        .await?;

    Ok(pool)
}

/// Run migrations. In real usage, you might keep .sql files in a migrations folder
/// and use sqlx's migrate capabilities. Or you can manually execute statements here.
/// For demonstration, let's do a quick "if not exists" approach.
pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<()> {
    let create_users_table = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            prompt TEXT
        );
    "#;

    let create_conversations_table = r#"
        CREATE TABLE IF NOT EXISTS conversations (
            user_id INTEGER PRIMARY KEY,
            conversation_summary TEXT,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );
    "#;

    sqlx::query(create_users_table).execute(pool).await?;
    sqlx::query(create_conversations_table)
        .execute(pool)
        .await?;

    Ok(())
}

/// Insert a new user into the DB, returning the created user
pub async fn create_user(pool: &Pool<Postgres>, name: &str, prompt: &str) -> Result<User> {
    let rec = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, prompt)
        VALUES ($1, $2)
        RETURNING id, name, prompt
        "#,
    )
    .bind(name)
    .bind(prompt)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

/// Retrieve a user by their ID
pub async fn get_user_by_id(pool: &Pool<Postgres>, user_id: i32) -> Result<Option<User>> {
    let rec = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, prompt
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(rec)
}

/// Retrieve a user by name
pub async fn get_user_by_name(pool: &Pool<Postgres>, name: &str) -> Result<Option<User>> {
    let rec = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, prompt
        FROM users
        WHERE name = $1
        "#,
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    Ok(rec)
}

pub async fn get_conversation_summary_by_user_id(
    pool: &Pool<Postgres>,
    user_id: i32,
) -> Result<Option<Conversation>> {
    let rec = sqlx::query_as::<_, Conversation>(
        r#"
        SELECT user_id, conversation_summary
        FROM conversations
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(rec)
}

/// Update a user's prompt
pub async fn update_user_prompt(
    pool: &Pool<Postgres>,
    user_id: i32,
    new_prompt: &str,
) -> Result<bool> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE users
        SET prompt = $1
        WHERE id = $2
        "#,
    )
    .bind(new_prompt)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(rows_affected.rows_affected() > 0)
}

/// Get or create user logic:
pub async fn get_or_create_user(
    pool: &Pool<Postgres>,
    name: &str,
    default_prompt: &str,
) -> Result<User> {
    if let Some(user) = get_user_by_name(pool, name).await? {
        return Ok(user);
    }
    // If user not found, create new user
    let new_user = create_user(pool, name, default_prompt).await?;
    Ok(new_user)
}

/// Get conversation summary by user_id
pub async fn get_conversation_summary(
    pool: &Pool<Postgres>,
    user_id: i32,
) -> Result<Option<String>> {
    let rec = sqlx::query_as::<_, (Option<String>,)>(
        r#"
        SELECT conversation_summary
        FROM conversations
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    // rec is an Option<(Option<String>,)>; flatten the outer Option
    Ok(rec.map(|tuple| tuple.0).flatten())
}

/// Update or insert (upsert) a conversation summary for a given user
pub async fn update_conversation_summary(
    pool: &Pool<Postgres>,
    user_id: i32,
    summary: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO conversations (user_id, conversation_summary)
        VALUES ($1, $2)
        ON CONFLICT (user_id)
        DO UPDATE SET conversation_summary = EXCLUDED.conversation_summary
        "#,
    )
    .bind(user_id)
    .bind(summary)
    .execute(pool)
    .await?;

    Ok(())
}
