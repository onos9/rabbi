use crate::{web, Error, Result};
use axum::{routing::post, Json, Router};
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, serde::Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: implement real db/auth logic.
    if payload.username != "Godsgrace" || payload.password != "password" {
        return Err(Error::LoginFailed);
    }

    // FIXME: Implement real auth token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = serde_json::json!({
        "data": null,
        "code": 200,
        "message": "Login successful",
        "success": true
    });

    Ok(Json(body))
}
