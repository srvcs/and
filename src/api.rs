use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utoipa::{OpenApi, ToSchema};

/// This service's identity. `srvcs-and` is a leaf: it depends on no other
/// service. Boolean AND is a self-contained logic primitive.
pub const SERVICE: &str = "srvcs-and";
pub const CONCERN: &str = "logic: boolean AND";
pub const DEPENDS_ON: &[&str] = &[];

#[derive(Serialize, ToSchema)]
pub struct Info {
    pub service: &'static str,
    pub concern: &'static str,
    pub depends_on: Vec<&'static str>,
}

/// `GET /` — service identity (srvcs service standard).
#[utoipa::path(get, path = "/", responses((status = 200, body = Info)))]
pub async fn index() -> Json<Info> {
    Json(Info {
        service: SERVICE,
        concern: CONCERN,
        depends_on: DEPENDS_ON.to_vec(),
    })
}

#[derive(Deserialize, ToSchema)]
pub struct EvalRequest {
    /// The first operand. Must be a JSON boolean.
    #[schema(value_type = Object)]
    pub a: Value,
    /// The second operand. Must be a JSON boolean.
    #[schema(value_type = Object)]
    pub b: Value,
}

#[derive(Serialize, ToSchema)]
pub struct AndResponse {
    #[schema(value_type = Object)]
    pub a: Value,
    #[schema(value_type = Object)]
    pub b: Value,
    pub result: bool,
}

/// The single concern: logical AND of two booleans.
pub fn and(a: bool, b: bool) -> bool {
    a && b
}

fn ok(a: Value, b: Value, result: bool) -> Response {
    (
        StatusCode::OK,
        Json(json!({ "a": a, "b": b, "result": result })),
    )
        .into_response()
}

fn invalid(reason: &str) -> Response {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(json!({ "error": reason })),
    )
        .into_response()
}

/// `POST /` — logical AND of `a` and `b`.
///
/// Both operands must be JSON booleans. `srvcs-and` is a self-contained leaf:
/// it validates its own input and depends on no other service.
#[utoipa::path(
    post,
    path = "/",
    request_body = EvalRequest,
    responses(
        (status = 200, body = AndResponse),
        (status = 422, description = "an operand is not a boolean")
    )
)]
pub async fn evaluate(Json(req): Json<EvalRequest>) -> Response {
    let Some(a) = req.a.as_bool() else {
        return invalid("a is not a boolean");
    };
    let Some(b) = req.b.as_bool() else {
        return invalid("b is not a boolean");
    };
    ok(req.a, req.b, and(a, b))
}

#[derive(OpenApi)]
#[openapi(
    paths(index, evaluate),
    components(schemas(Info, EvalRequest, AndResponse))
)]
pub struct ApiDoc;

/// Serve OpenAPI document
pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_documents_routes() {
        let doc = ApiDoc::openapi();
        let root = doc.paths.paths.get("/").expect("path / present");
        assert!(root.get.is_some(), "GET / documented");
        assert!(root.post.is_some(), "POST / documented");
    }

    #[test]
    fn truth_table_is_correct() {
        assert!(and(true, true));
        assert!(!and(true, false));
        assert!(!and(false, true));
        assert!(!and(false, false));
    }

    #[test]
    fn index_reports_identity() {
        assert_eq!(SERVICE, "srvcs-and");
        assert!(DEPENDS_ON.is_empty());
    }
}
