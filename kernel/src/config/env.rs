use std::{str::FromStr, time::Duration};

use serde::{Deserialize, Serialize};

use crate::{errors::AppError, utils::extract_env};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_environment")]
    pub environment: Environment,

    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,

    #[serde(default = "default_requests_time_out")]
    pub requests_time_out_secs: Duration,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        tracing::info!("Loading application configuration...");

        let requests_time_out = extract_env::<u64>("REQUESTS_TIME_OUT_SECS")
            .unwrap_or_else(|_| default_requests_time_out().as_secs());

        Ok(Self {
            // Server
            port: extract_env("PORT").unwrap_or_else(|_| default_port()),
            environment: extract_env::<Environment>("ENVIRONMENT")
                .unwrap_or_else(|_| default_environment()),

            // CORS
            allowed_origins: extract_env::<String>("ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".into())
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned)
                .collect(),

            requests_time_out_secs: Duration::from_secs(requests_time_out),
        })
    }

    pub fn current_env() -> Environment {
        match std::env::var("ENVIRONMENT")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "development" | "dev" => Environment::Development,
            "production" | "prod" => Environment::Production,
            _ => Environment::Development,
        }
    }

    pub fn is_production(&self) -> bool {
        matches!(self.environment, Environment::Production)
    }

    pub fn is_development(&self) -> bool {
        matches!(self.environment, Environment::Development)
    }
}

pub fn load_env() -> Result<AppConfig, AppError> {
    dotenvy::dotenv().ok();
    AppConfig::from_env()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            _ => Err(format!("Invalid environment: {value}")),
        }
    }
}

fn default_port() -> u16 {
    8000
}

fn default_environment() -> Environment {
    Environment::Development
}

fn default_allowed_origins() -> Vec<String> {
    vec![
        "http://localhost:3000".into(),
        "http://localhost:8000".into(),
    ]
}

fn default_requests_time_out() -> Duration {
    Duration::from_secs(10)
}
