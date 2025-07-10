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

/// Validates passwords in register requests.
///
/// A valid password:
/// - is between 12-20 characters
/// - contains at least one special character
/// - contains at least one number
fn validate_password(pass: &str) -> bool {
    const ALLOWED_SPECIALS: [char; 30] = [
        '@', '#', '!', '?', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '{', '}', '[',
        ']', '|', '\\', ':', ';', '<', '>', ',', '.', '/', '~', '`',
    ];

    let mut chars = pass.chars();
    let mut special: bool = false;
    let mut number: bool = false;

    while !special
        && !number
        && let Some(c) = chars.next()
    {
        if c.is_alphabetic() {
            continue; // maybe check for uppercase in future?
        } else if c.is_numeric() {
            number = true; // found at least 1 number
        } else if ALLOWED_SPECIALS.contains(&c) {
            special = true; // found at least 1 special character
        } else {
            return false;
        }
    }

    pass.len() >= 12 && pass.len() <= 20 && special && number
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
            tracing::debug!(name: "Register Request", email = reg_req.email, password = reg_req.password);

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
