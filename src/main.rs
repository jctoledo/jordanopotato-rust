use anyhow::Result;
use dotenv::dotenv;
use std::net::SocketAddr;

use axum::Router;
use jordanopotato_rust::{db, routes};
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // 1. Initialize DB
    let pool: Pool<Postgres> = db::init_db_pool().await?;
    db::run_migrations(&pool).await?;
    println!("Database connected and migrations applied successfully!");

    // 2. Build the Axum router
    let app: Router = routes::create_router(pool);

    // 3. Run Axum
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
