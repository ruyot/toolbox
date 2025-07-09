use std::sync::LazyLock;

use axum::{
    body::Bytes,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use regex::{Regex, RegexBuilder};
use rmp_serde::decode;
use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

fn validate_password(pass: &str) -> bool {
    // TODO: add more validation later
    pass.len() >= 8
}

static EMAIL_VALID: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r#"^[^@\s]*?@[^@\s]*?\.[^@\s]*$"#)
        .multi_line(true)
        .build()
        .unwrap()
});

pub async fn register(payload: Bytes) -> Response {
    // decode what is hopefully MessagePack payload
    match decode::from_slice::<RegisterRequest>(&payload) {
        Ok(reg_req) => {
            // validate email & pass
            if !EMAIL_VALID.is_match(&reg_req.email) || !validate_password(&reg_req.password) {
                return (StatusCode::BAD_REQUEST, "Invalid email or password format")
                    .into_response();
            }

            // TODO: save user to database

            StatusCode::CREATED.into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Invalid request format").into_response(),
    }
}
