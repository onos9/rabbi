#![allow(unused)]

use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use ctx::Ctx;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    log::log_request,
    model::ModelController,
};

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[derive(Debug, serde::Deserialize)]
struct Params {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;
    let api_routes = web::tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::auth::mw_require_auth));

    let app = Router::new()
        .merge(hello_routes())
        .nest("/api", api_routes)
        .layer(middleware::map_response(response_maper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::auth::mw_ctx_resolver,
        ))
        .merge(web::login::routes())
        .layer(CookieManagerLayer::new())
        .fallback_service(public_routes());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn hello_routes() -> Router {
    Router::new()
        .route("/hello", get(root))
        .route("/hello/:name", get(greet))
}

async fn response_maper(ctx: Option<Ctx>, uri: Uri, req_method: Method, res: Response) -> Response {
    println!("->> {:<12} - response_maper", "RES_MAPER");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>().cloned();
    let client_status = service_error.map(|e| e.client_status());

    // If client error, build a new response
    let resp_error = client_status.as_ref().map(|(status_code, client_error)| {
        let error_body = json!({
            "error": {
                "uuid": uuid.to_string(),
                "code": status_code.as_u16(),
                "type": client_error.as_ref(),
                "success": false
            },
        });
        println!("  ->> CLIENT_ERROR: {error_body:?}");
        // Build the new response with the error body
        (*status_code, Json(error_body)).into_response()
    });

    // TODO: Build and log the server logline
    let client_error = client_status.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    resp_error.unwrap_or(res)
}

async fn root(Query(params): Query<Params>) -> impl IntoResponse {
    println!("->> {:<12} - root", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello, {name}!</h1>"))
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - greet", "HANDLER");
    Html(format!("<h1>Hello, {name}!</h1>"))
}

fn public_routes() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}
