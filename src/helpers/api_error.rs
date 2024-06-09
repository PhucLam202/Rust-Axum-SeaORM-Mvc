use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub struct APIerror {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for APIerror {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (status_code,[(header::CONTENT_TYPE,"application/json")], Json(json!({ 
            "StatusCode": self.status_code.as_u16(),
            "Message": self.message })) 
        ).into_response()
    }
}
