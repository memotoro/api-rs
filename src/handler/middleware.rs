use crate::model::error::AppError;
use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue, Request},
    middleware::Next,
    response::IntoResponse,
};

async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, AppError> {
    let mut istio_headers: HeaderMap<HeaderValue> = HeaderMap::new();

    let (parts, body) = req.into_parts();

    for key in istio_header_keys() {
        for (h, v) in parts.headers.iter() {
            if h.to_string() == key {
                istio_headers.append(h, v.clone());
            }
        }
    }

    let req = Request::from_parts(parts, body);

    let res = next.run(req).await;

    Ok(res)
}

fn istio_header_keys() -> Vec<String> {
    vec![
        "x-b3-request".to_string(),
        "x-b3-trace".to_string(),
        "x-b3-span".to_string(),
    ]
}
