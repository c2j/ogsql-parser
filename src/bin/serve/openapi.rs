//! OpenAPI specification configuration.

use utoipa::OpenApi;

use super::handlers;
use super::schema;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health,
        handlers::handle_parse,
        handlers::handle_format,
        handlers::handle_tokenize,
        handlers::handle_validate,
        handlers::handle_json2sql,
    ),
    components(
        schemas(
            schema::ParseInput,
            schema::FormatInput,
            schema::ValidateInput,
            schema::TokenizeInput,
            schema::JsonInput,
            schema::LintConfigInput,
            schema::HealthResponse,
            schema::FormatResponse,
            schema::TokenizeResponse,
            schema::TokenInfo,
            schema::Json2SqlResponse,
            schema::ParseResponse,
            schema::ValidateResponse,
            super::error::ApiErrorBody,
        )
    ),
    tags(
        (name = "ogsql", description = "openGauss/GaussDB SQL Parser API")
    )
)]
pub struct ApiDoc;

#[cfg(feature = "ibatis")]
#[derive(OpenApi)]
#[openapi(
    paths(handlers::handle_parse_xml, handlers::handle_validate_xml),
    components(schemas(schema::ParseXmlInput, schema::ValidateXmlInput)),
    tags((name = "ogsql", description = "iBatis/MyBatis XML parsing"))
)]
pub struct ApiDocIbatis;

#[cfg(feature = "java")]
#[derive(OpenApi)]
#[openapi(
    paths(handlers::handle_parse_java, handlers::handle_validate_java),
    components(schemas(schema::ParseJavaInput, schema::ValidateJavaInput)),
    tags((name = "ogsql", description = "Java SQL extraction"))
)]
pub struct ApiDocJava;

/// Build merged OpenAPI spec from all feature-gated ApiDoc structs.
pub fn build_openapi() -> utoipa::openapi::OpenApi {
    #[allow(unused_mut)]
    let mut spec = ApiDoc::openapi();
    #[cfg(feature = "ibatis")]
    {
        spec.merge(ApiDocIbatis::openapi());
    }
    #[cfg(feature = "java")]
    {
        spec.merge(ApiDocJava::openapi());
    }
    spec
}
