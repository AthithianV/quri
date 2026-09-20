use anyhow::{Context, Result};
use dirs::data_dir;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::{Path, PathBuf};

pub async fn initialize_local_db() -> Result<SqlitePool> {
    let app_data_path: PathBuf = data_dir().context("Cannot find app data dir")?;

    let db_dir = Path::new(&app_data_path).join("quri");
    if !db_dir.exists() {
        std::fs::create_dir_all(&db_dir)
            .with_context(|| format!("Failed to create database directory: {:?}", db_dir))?;
    }

    let db_path = Path::new(&db_dir).join("data.db");
    let db_exists = db_path.exists();
    println!("Database exists: {}", db_exists);

    let db_url = format!("sqlite://{}/data.db?mode=rwc", db_dir.to_str().unwrap());
    println!("Connecting to database with URL: {}", db_url);

    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .context("Failed to connect to database")?;

    // Only run migrations if database doesn't exist or has pending migrations
    if !db_exists {
        sqlx::raw_sql(include_str!("../migration/2026-09-20_intial.sql"))
            .execute(&db)
            .await
            .context("Schema initialization failed while creating new DB")?;
    } else {
        println!("Database already exists, no schema migration needed");
    }

    println!("Database initialization completed");
    Ok(db)
}
