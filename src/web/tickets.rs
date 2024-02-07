use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

use axum::routing::{delete, post};
use axum::{
    extract::{Path, State},
    Json, Router,
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    ctx: Ctx,
    State(model): State<ModelController>,
    Json(payload): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    let ticket = model.create_ticket(ctx,payload).await?;
    Ok(Json(ticket))
}
async fn list_tickets(ctx: Ctx, State(model): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = model.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    ctx: Ctx,
    State(model): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let ticket = model.delete_ticket(ctx,id).await?;
    Ok(Json(ticket))
}
