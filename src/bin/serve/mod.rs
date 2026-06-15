//! HTTP API server module — router, middleware, and Swagger UI.

pub mod error;
pub mod handlers;
pub mod openapi;
pub mod schema;

use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

#[cfg(feature = "utoipa-swagger-ui")]
use utoipa_swagger_ui::SwaggerUi;

const MAX_BODY_SIZE_BYTES: usize = 10 * 1024 * 1024;

/// Build the axum Router with all routes, middleware, and Swagger UI.
///
/// When compiled with the `serve` feature, Swagger UI assets (CSS/JS/HTML)
/// are embedded at compile time by `utoipa-swagger-ui` — no internet access
/// required at runtime.
#[allow(clippy::future_not_send)]
pub fn router() -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let trace_layer = TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
        let request_id = request.headers().get("x-request-id").and_then(|v| v.to_str().ok()).unwrap_or("-");
        tracing::info_span!(
            "http_request",
            request_id = %request_id,
            method = %request.method(),
            uri = %request.uri(),
        )
    });

    #[allow(unused_mut)]
    let mut router = Router::new()
        .route("/api/health", get(handlers::health))
        .route("/api/parse", post(handlers::handle_parse))
        .route("/api/json2sql", post(handlers::handle_json2sql))
        .route("/api/format", post(handlers::handle_format))
        .route("/api/tokenize", post(handlers::handle_tokenize))
        .route("/api/validate", post(handlers::handle_validate))
        .route("/api-docs/openapi.json", get(|| async { Json(openapi::build_openapi()) }))
        .layer((SetRequestIdLayer::x_request_id(MakeRequestUuid), trace_layer, PropagateRequestIdLayer::x_request_id()))
        .layer(DefaultBodyLimit::max(MAX_BODY_SIZE_BYTES))
        .layer(cors);

    #[cfg(feature = "utoipa-swagger-ui")]
    {
        let openapi = openapi::build_openapi();
        router = router.merge(
            SwaggerUi::new("/api-docs/swagger-ui").url("/api-docs/openapi.json", openapi),
        );
    }

    #[cfg(feature = "ibatis")]
    {
        router = router
            .route("/api/parse-xml", post(handlers::handle_parse_xml))
            .route("/api/validate-xml", post(handlers::handle_validate_xml));
    }
    #[cfg(feature = "java")]
    {
        router = router
            .route("/api/parse-java", post(handlers::handle_parse_java))
            .route("/api/validate-java", post(handlers::handle_validate_java));
    }

    router
}
