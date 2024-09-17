use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use profile::get_profile;
use secrecy::{ExposeSecret, Secret};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::{environment::Environment, AppState};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

mod profile;

#[utoipauto]
#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub fn app_router(
    env: &Environment,
    app_state: AppState,
    cors_allow_origin: &Secret<String>,
) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin([cors_allow_origin.expose_secret().parse().unwrap()])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    let layers = ServiceBuilder::new().layer(cors_layer);

    let router = Router::new()
        .route("/profile", get(get_profile))
        .with_state(app_state)
        .layer(layers)
        .route("/", get(|| async { Html("<h1>Hello World!</h1>") }))
        .route("/health_check", get(|| async { StatusCode::OK }))
        .fallback(|| async { (StatusCode::NOT_FOUND, "invalid api") });

    match env {
        Environment::Local => {
            router.merge(SwaggerUi::new("/docs").url("/docs.json", ApiDoc::openapi()))
        }
        Environment::Production => router,
    }
}
