//! Simplistic Model Layer
//! (with mock-store layer for testing)

use crate::{
    ctx::{self, Ctx},
    error::{Error, Result},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Clone, Debug)]
pub struct Ticket {
    pub id: u64,
    cid: u64, // creator user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticket_store: Arc::default(),
        })
    }

    pub async fn create_ticket(&self, ctx: Ctx, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets = store.iter().filter_map(|ticket| ticket.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|ticket| ticket.take());

        ticket.ok_or(Error::TicketDeletionFailedNotFound { id })
    }
}
