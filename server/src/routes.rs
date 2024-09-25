use axum::{
    body::Body,
    extract::Request,
    http::{header::CONTENT_TYPE, Response, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use resume::get_resume;
use secrecy::{ExposeSecret, SecretString};
use settings::Environment;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::Span;

use crate::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

mod resume;

#[utoipauto(paths = "./server/src")]
#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub fn app_router(
    env: &Environment,
    app_state: AppState,
    cors_allow_origin: &SecretString,
) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin([cors_allow_origin.expose_secret().parse().unwrap()])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let request_id = uuid::Uuid::new_v4();
            tracing::info_span!(
                "request",
                method = tracing::field::display(request.method()),
                uri = tracing::field::display(request.uri()),
                version = tracing::field::debug(request.version()),
                request_id = tracing::field::display(request_id),
                latency = tracing::field::Empty,
                status_code = tracing::field::Empty,
            )
        })
        .on_response(
            |response: &Response<Body>, latency: std::time::Duration, span: &Span| {
                span.record("status_code", tracing::field::display(response.status()));
                span.record("latency", tracing::field::debug(latency));
                // add tracing below here
                // useful if using bunyan trace format
            },
        );

    let layers = ServiceBuilder::new().layer(trace_layer).layer(cors_layer);

    let router = Router::new()
        .route("/resume", get(get_resume))
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
