use serde::Deserialize;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .add_source(File::with_name("config/config").required(false));

        // Check for environment-specific config
        if let Ok(env) = std::env::var("RUST_ENV") {
            builder = builder.add_source(File::with_name(&format!("config/config.{}", env)).required(false));
        }

        // Set default values
        builder = builder
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 10001)?;

        let config = builder.build()?;
        config.try_deserialize()
    }
}
