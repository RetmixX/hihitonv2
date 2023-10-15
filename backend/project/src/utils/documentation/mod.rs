use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::healthcheck
    ),
    components(
        schemas(
            super::responses::MessageResponse,
            super::error_handlers::api_error_structs::ApiErrorResponse
        )
    )
)]
pub struct SwaggerDoc;