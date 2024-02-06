use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::{
    error::{Error, Result},
    model::ModelController,
};

mod error;
mod model;
mod web;
mod ctx;

#[derive(Debug, serde::Deserialize)]
struct Params {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;
    let api_routes = web::tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::auth::require_auth));

    let app = Router::new()
        .merge(hello_routes())
        .merge(web::login::routes())
        .nest("/api", api_routes)
        .layer(middleware::map_response(response_maper))
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

async fn response_maper(res: Response) -> Response {
    println!("->> {:<12} - response_maper", "RES_MAPER");
    println!();
    res
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
