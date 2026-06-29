use anyhow::{Context, Result};
use dirs::data_dir;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::path::{Path, PathBuf};

pub async fn initialize_local_db() -> Result<DatabaseConnection> {
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

    let db: DatabaseConnection = Database::connect(&db_url)
        .await
        .context("Failed to connect to database")?;

    // Only run migrations if database doesn't exist or has pending migrations
    if !db_exists {
        Migrator::up(&db, None)
            .await
            .context("Migration failed while creating new DB")?;
    } else {
        let pending = Migrator::get_pending_migrations(&db)
            .await
            .context("Failed to check pending migrations")?;

        if !pending.is_empty() {
            println!(
                "Found {} pending migrations, running them...",
                pending.len()
            );
            Migrator::up(&db, None)
                .await
                .context("Migration failed while applying pending migrations")?;
        } else {
            println!("Database is up to date, no migrations needed");
        }
    }

    println!("Database initialization completed");
    Ok(db)
}
