use actix_web::{
    HttpRequest, HttpResponse, Result, get, post,
    web::{self, Form, Html},
};
use serde::{Deserialize, Serialize};

use crate::{api::auth::StatusReader, server::ServerState};

#[get("/api/health")]
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/api/status")]
async fn status(
    _auth: StatusReader,
    app: web::Data<ServerState>,
) -> Result<HttpResponse> {
    todo!()
}
