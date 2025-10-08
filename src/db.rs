use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use sqlx::{Error as SqlxError, migrate::MigrateDatabase, Sqlite};
use std::time::Duration;
use std::str::FromStr;

pub async fn create_database_pool(database_url: &str) -> Result<SqlitePool, SqlxError> {
    // Parse connection options with proper configuration
    let connect_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(30))
        // .disable_statement_logging() // Disable logging for sensitive data
        ; 

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Some(Duration::from_secs(600)))
        .max_lifetime(Some(Duration::from_secs(3600)))
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), SqlxError> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub async fn setup_database(database_url: &str) -> Result<SqlitePool, SqlxError> {
    // Create database if it doesn't exist
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        println!("Creating database {}", database_url);
        match Sqlite::create_database(database_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    // Create connection pool
    let pool = create_database_pool(database_url).await?;
    
    // Run migrations
    println!("Running database migrations...");
    run_migrations(&pool).await?;
    println!("Migrations completed successfully");

    Ok(pool)
}