pub mod config;
pub mod console;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod repositories;
pub mod routes;

pub mod db {
    use crate::error::{Error, Result};
    use sqlx::{postgres::PgPoolOptions, PgPool};

    pub type DbPool = PgPool;

    pub async fn establish_connection(database_url: &str) -> Result<DbPool> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(pool)
    }

    pub async fn run_migrations(db_pool: &PgPool) -> Result<()> {
        sqlx::migrate!().run(db_pool).await.map_err(|err| {
            tracing::error!("{err}");
            Error::InternalServerError
        })?;

        Ok(())
    }
}

pub mod logging {
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    use crate::config::LogFormat;

    pub fn init_tracing(log_level: &str, log_format: &LogFormat) {
        let reg = tracing_subscriber::registry();

        let reg = reg.with(EnvFilter::builder().parse_lossy(log_level));

        match log_format {
            super::config::LogFormat::Full => reg.with(fmt::layer()).init(),
            super::config::LogFormat::JSON => reg.with(fmt::layer().json()).init(),
            super::config::LogFormat::Pretty => reg.with(fmt::layer().pretty()).init(),
            super::config::LogFormat::Compact => reg.with(fmt::layer().compact()).init(),
        };
    }
}
