use crate::model::domain::{ComplexData, SimpleData};
use crate::model::error::AppError;
use crate::model::state::AppState;
use anyhow::Result;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use reqwest::Client;
use tracing::error;

pub async fn root(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let name = state.api_name;

    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    let simple_data = SimpleData { name, timestamp };

    Ok((StatusCode::OK, Json(simple_data)))
}

pub async fn proxy(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let mut data: Vec<SimpleData> = Vec::new();

    let client = create_client(headers)?;

    for api in state.apis {
        let response = client.get(api).send().await.map_err(|e| {
            error!("{}", e);
            AppError::ApplicationError("error calling service".to_string())
        })?;

        let simple_data = response.json::<SimpleData>().await.map_err(|e| {
            error!("{}", e);
            AppError::ApplicationError("error marshalling response".to_string())
        })?;

        data.push(simple_data);
    }

    let name = state.api_name;

    let complex_data = ComplexData { name, data };

    Ok((StatusCode::OK, Json(complex_data)))
}

fn create_client(headers: HeaderMap) -> Result<Client, AppError> {
    let mut header_map: HeaderMap = HeaderMap::new();

    for key in istio_header_keys() {
        if let Some(v) = headers.get(key) {
            let value = v.clone();
            header_map.append(key, value);
        }
    }

    let client = reqwest::ClientBuilder::new()
        .default_headers(header_map)
        .build()
        .map_err(|e| {
            error!("{}", e);
            AppError::ApplicationError("error building client".to_string())
        })?;

    Ok(client)
}

fn istio_header_keys() -> Vec<&'static str> {
    vec!["x-request-id", "x-b3-traceid", "x-b3-spanid"]
}
