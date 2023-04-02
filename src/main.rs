use api_rs::config::api::Config;
use api_rs::handler::processor::{
    failure_processor, request_processor, response_processor, span_processor,
};
use api_rs::handler::server::{proxy, root};
use api_rs::model::state::AppState;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = Config::new_from_env().unwrap();

    let api_name = config.clone().api_name;

    let apis = config.apis();

    let state = AppState { api_name, apis };

    let app = Router::new()
        .route("/", get(root))
        .route("/apis", get(proxy))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(span_processor)
                .on_request(request_processor)
                .on_response(response_processor)
                .on_failure(failure_processor),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
