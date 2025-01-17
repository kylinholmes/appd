use std::path::PathBuf;

use axum::{
    http::{HeaderMap, StatusCode},
    Json, Router,
};

use config::{CONFIG, CONFIG_PATH};
use error::AppdError;
use log::info;
use miniapp::mini_app_api;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};

mod miniapp;
mod config;
mod error;
mod file;

pub fn get_api(cfg: PathBuf) -> Router {
    CONFIG_PATH.set(cfg).unwrap();
    info!("{:?}", *CONFIG);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(appd_api())
        .split_for_parts();


    if cfg!(debug_assertions) {
        info!("debug mode, enable openapi with scalar");
        router.merge(Scalar::with_url("/", api))
    } else {
        router
    }
}

fn appd_api() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(ping))
        .merge(mini_app_api())
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = "appd", description = "mini-app mangment api",)
    )
)]
struct ApiDoc;
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("api_key"))),
            )
        }
    }
}

#[utoipa::path(
    get,
    path = "/ping",
    tags = ["appd"],
    responses(
        (status = 200, description = "ping-pong", body = String)
    )
)]
async fn ping() -> &'static str {
    "pong"
}

fn check_api_key(
    require_api_key: bool,
    headers: HeaderMap,
) -> Result<(), (StatusCode, Json<AppdError>)> {
    match headers.get("api_key") {
        Some(header) if header != "utoipa-rocks" => Err((
            StatusCode::UNAUTHORIZED,
            Json(AppdError::Unauthorized(String::from("incorrect api key"))),
        )),
        None if require_api_key => Err((
            StatusCode::UNAUTHORIZED,
            Json(AppdError::Unauthorized(String::from("missing api key"))),
        )),
        _ => Ok(()),
    }
}
