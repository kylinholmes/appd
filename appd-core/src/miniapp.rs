use serde::{Deserialize, Serialize};
use utoipa::{schema, IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::IntoResponse,
    Json,
};

use crate::check_api_key;

pub fn mini_app_api() -> OpenApiRouter {
    OpenApiRouter::new().nest(
        "/app",
        OpenApiRouter::new().routes(routes!(get_apps, upload_app, delete_app)), // .routes(routes!(upload_app))
    )
}

/// Item to do.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct App {
    id: i32,
    #[schema(example = "http://127.0.0.1/app/{id}")]
    url: String,

    #[schema(example = "/data/app/{id}/{version}")]
    actual_path: String,
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
struct AppSearchQuery {
    keyword: String,
    version: Option<String>,
}

#[utoipa::path(
    get,
    path = "/",
    tags = ["miniapp"],
    params(
        AppSearchQuery
    ),
    responses(
        (status = 200, description = "query app", body = [App])
    )
)]
async fn get_apps(Query(query): Query<AppSearchQuery>) -> Json<Vec<App>> {
    Json(vec![App {
        id: 1,
        url: String::from("123123"),
        actual_path: String::from("123123"),
    }])
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
struct AppUploadQ {
    appinfo: App,
}

#[utoipa::path(
    post,
    path = "/",
    tags = ["miniapp"],
    params(
        AppUploadQ
    ),
    responses(
        (status = 200, description = "upload a app", body = App)
    )
)]
async fn upload_app(Query(query): Query<AppUploadQ>) -> Json<App> {
    Json(query.appinfo)
}

// delete
#[utoipa::path(
    delete,
    path = "/{id}",
    tags = ["miniapp"],
    responses(
        (status = 200, description = "delete a app", body = String)
    )
)]
async fn delete_app(Path(id): Path<i32>, headers: HeaderMap) -> impl IntoResponse {
    match check_api_key(true, headers) {
        Ok(_) => (),
        Err(error) => return error.into_response(),
    };
    "ok".into_response()
}
