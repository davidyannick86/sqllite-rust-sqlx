use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};
use std::result::Result;

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;
    let query = "
    PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS settings (
        settings_id INTEGER PRIMARY KEY NOT NULL,
        descritpion TEXT NOT NULL,
        created_on DATETIME DEFAULT (datetime('now','localtime')),
        updated_on DATETIME DEFAULT (datetime('now','localtime')),
        done BOOLEAN NOT NULL DEFAULT 0
    );
    CREATE TABLE IF NOT EXISTS project (
        project_id INTEGER PRIMARY KEY AUTOINCREMENT,
        product_name TEXT,
        created_on DATETIME DEFAULT (datetime('now','localtime')),
        updated_on DATETIME DEFAULT (datetime('now','localtime')),
        settings_id INTEGER NOT NULL DEFAULT 1,
        FOREIGN KEY (settings_id) REFERENCES settings(settings_id) ON UPDATE SET NULL ON DELETE SET NULL
    );";

    let result = sqlx::query(query).execute(&pool).await;
    pool.close().await;
    result
}

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Schema created successfully"),
            Err(e) => panic!("Error creating schema: {}", e),
        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let query = "INSERT INTO settings(descritpion) VALUES($1)";
    let result = sqlx::query(query).bind("Test").execute(&instances).await;

    instances.close().await;

    println!("Result: {:?}", result);
}
