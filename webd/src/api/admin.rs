use axum::{
    http::{HeaderMap, StatusCode},
    Json, Router,
};
use once_cell::sync::{Lazy, OnceCell};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use crate::utils::VERSION;

use super::WebdError;

pub static ADMIN_ROUTER: Lazy<Router> = Lazy::new(Router::new);
static API_TOKEN: OnceCell<String> = OnceCell::new();

pub fn admin_api() -> Router {
    let tk = gen_random_token();
    println!("apitoken: {}", tk);
    API_TOKEN.set(tk).unwrap();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(api_router())
        .split_for_parts();

    router.merge(Scalar::with_url("/scalar", api))
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = "Webd", description = "Webd management",)
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

fn api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(ping, sysinfo))
        .routes(routes!(version))
    // .routes(routes!(login))
}

pub fn check_api_key(
    require_api_key: bool,
    headers: HeaderMap,
) -> Result<(), (StatusCode, Json<WebdError>)> {
    match headers.get("api_key") {
        Some(header) if header != "utoipa-rocks" => Err((
            StatusCode::UNAUTHORIZED,
            Json(WebdError::Unauthorized(String::from("incorrect api key"))),
        )),
        None if require_api_key => Err((
            StatusCode::UNAUTHORIZED,
            Json(WebdError::Unauthorized(String::from("missing api key"))),
        )),
        _ => Ok(()),
    }
}

#[utoipa::path(
    get,
    path = "/ping",
    tags = ["admin"],
    responses(
        (status = 200, description = "ping-pong", body = String)
    )
)]
pub async fn ping() -> &'static str {
    "pong"
}

#[utoipa::path(
    get,
    path = "/version",
    tags = ["admin"],
    responses(
        (status = 200, description = "get webd version", body = String)
    )
)]
pub async fn version() -> &'static str {
    VERSION.get().unwrap()
}

#[utoipa::path(
    post,
    path = "/sysinfo",
    tags = ["admin"],
    responses(
        (status = 200, description = "get runtime hardware sysinfo", body = String)
    )
)]
pub async fn sysinfo() {}

#[utoipa::path(
    post,
    path = "/login",
    tags = ["admin"],
    responses(
        (status = 200, description = "login", body = String)
    )
)]
pub async fn login() {}

pub fn gen_random_token() -> String {
    String::from("abcabc")
}
