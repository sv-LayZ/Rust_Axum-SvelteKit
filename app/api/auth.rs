use axum::{response::IntoResponse, Json};

pub async fn route_handler() -> impl IntoResponse {
    const MESSAGE: &str = "";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
