#![forbid(unsafe_code)]

use anyhow::Context;
use axum::{
    extract::FromRef,
    http::{header::AUTHORIZATION, HeaderValue},
    routing::get,
    Json, Router,
};
use config::Config;
use maglev::EnvConfig;
use openai::OpenAiClient;
use serde_json::{json, Value};
use spiceai::ClientBuilder;
use std::{net::Ipv4Addr, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer, cors,
    sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

mod config;
mod error;
mod openai;
mod serde_utils;
mod v1;

pub(crate) type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Clone, FromRef)]
pub struct Ctx {
    pub config: Arc<Config>,
    pub spice: Arc<Mutex<spiceai::Client>>,
    pub openai: OpenAiClient,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;
    let port = config.port;
    tracing::trace!(
        "Initialize Spice client: {}",
        config.spice_flight_url.as_str()
    );
    let spice = ClientBuilder::new()
        .flight_url(config.spice_flight_url.as_str())
        .build()
        .await
        .unwrap();
    let openai = OpenAiClient::with_base_url(config.spice_http_url.join("v1").unwrap().as_str());

    let ctx = Ctx {
        config: Arc::new(config),
        spice: Arc::new(Mutex::new(spice)),
        openai,
    };

    let app = api_router(ctx);
    let addr = (Ipv4Addr::UNSPECIFIED, port);
    maglev::serve(addr, app)
        .await
        .context("error running HTTP server")
}

fn api_router(ctx: Ctx) -> Router {
    let cors_layer = match &ctx.config.allow_origin {
        None => cors::CorsLayer::new(),
        Some(allow_origin) => {
            let origin = allow_origin
                .origin()
                .ascii_serialization()
                .parse::<HeaderValue>()
                .map(cors::AllowOrigin::exact)
                .unwrap();
            cors::CorsLayer::new()
                .allow_origin(origin)
                .allow_headers(cors::AllowHeaders::mirror_request())
                .allow_methods(cors::AllowMethods::mirror_request())
                .allow_credentials(true)
        }
    };

    Router::new()
        .merge(v1::router())
        .layer((
            SetSensitiveHeadersLayer::new([AUTHORIZATION]),
            CompressionLayer::new(),
            TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
                tracing::info_span!("request", method = %request.method(), uri = %request.uri(), user = tracing::field::Empty)
            }),
            cors_layer,
            TimeoutLayer::new(Duration::from_secs(30)),
            CatchPanicLayer::new(),
        ))
        .with_state(ctx)
        // Health endpoint without above middleware (i.e. disable tracing)
        .route("/health", get(health))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}
