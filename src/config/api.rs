use crate::model::error::AppError;
use anyhow::Result;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub api_name: String,
    pub api_csv: String,
    pub port: u16,
}

impl Config {
    pub fn new_from_env() -> Result<Self, AppError> {
        envy::from_env().map_err(|e| {
            error!("{}", e);
            AppError::ApplicationError("error reading config".to_string())
        })
    }

    pub fn apis(&self) -> Vec<String> {
        self.api_csv
            .split(',')
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
    }
}
