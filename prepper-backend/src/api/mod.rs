use crate::server::ServerState;
use actix_web::{
    HttpRequest, HttpResponse, Result, get, post,
    web::{self, Form, Html},
};
use serde::{Deserialize, Serialize};

pub mod app;
pub mod auth;
pub mod health;
pub mod products;

#[get("/")]
async fn index() -> Result<Html> {
    todo!()
}
