use std::future::{Ready, ready};

use crate::server::ServerState;
use actix_web::{
    Error, FromRequest, HttpRequest, HttpResponse, Result,
    error::InternalError,
    get,
    http::header::LOCATION,
    post,
    web::{self, Form, Html},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

pub const TOKEN: &'static str = "token";
pub const PRODUCTS_SCOPE: &'static str = "products:write";
pub const STATUS_SCOPE: &'static str = "status:read";

pub fn redirect(to: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, to))
        .finish()
}

pub fn to_login() -> Error {
    InternalError::from_response("Unauthorized", redirect("/login")).into()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductWriter {
    claims: Claims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusReader {
    claims: Claims,
}

impl FromRequest for StatusReader {
    type Error = actix_web::error::Error;

    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let secret = req
            .app_data::<web::Data<ServerState>>()
            .expect("State is missing")
            .secret
            .as_bytes();
        let token = req.cookie(TOKEN).map(|c| c.value().to_string());

        let Some(token) = token else {
            return ready(Err(to_login()));
        };

        let key = match DecodingKey::from_secret(secret) {
            Ok(k) => k,
            Err(_) => return ready(Err(to_login())),
        };

        let data = match decode::<Claims>(token, &key, &Validation::default()) {
            Ok(d) => d,
            Err(_) => return ready(Err(to_login())),
        };

        if !data.claims.scope.split(" ").any(|s| s == STATUS_SCOPE) {
            return ready(Err(to_login()));
        }

        ready(Ok(StatusReader {
            claims: data.claims,
        }))
    }
}

impl FromRequest for ProductWriter {
    type Error = actix_web::error::Error;

    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let secret = req
            .app_data::<web::Data<ServerState>>()
            .expect("State is missing")
            .secret
            .as_bytes();
        let token = req.cookie(TOKEN).map(|c| c.value().to_string());

        let Some(token) = token else {
            return ready(Err(to_login()));
        };

        let key = match DecodingKey::from_secret(secret) {
            Ok(k) => k,
            Err(_) => return ready(Err(to_login())),
        };

        let data = match decode::<Claims>(token, &key, &Validation::default()) {
            Ok(d) => d,
            Err(_) => return ready(Err(to_login())),
        };

        if !data.claims.scope.split(" ").any(|s| s == PRODUCTS_SCOPE) {
            return ready(Err(to_login()));
        }

        ready(Ok(ProductWriter {
            claims: data.claims,
        }))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub scope: String,
}

#[post("/auth/login")]
async fn login(
    req: HttpRequest,
    app: web::Data<ServerState>,
    web::Form(form): Form<LoginForm>,
) -> Result<HttpResponse> {
    todo!()
}

#[post("/auth/logout")]
async fn logout() -> Result<HttpResponse> {
    todo!()
}
