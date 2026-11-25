use crate::{api::auth::ProductWriter, server::ServerState};
use actix_web::{
    HttpRequest, HttpResponse, Result, delete, get, post, put,
    web::{self, Form, Html},
};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewProductForm {}

#[post("/api/product/new")]
async fn new_product(
    auth: ProductWriter,
    req: HttpRequest,
    app: web::Data<ServerState>,
    web::Form(form): Form<NewProductForm>,
) -> Result<Html> {
    todo!()
}

#[derive(Template)]
#[template(path = "new_product.html")]
pub struct NewProductTemplate;

#[get("/product/new")]
async fn new_product_page(auth: ProductWriter) -> Result<Html> {
    Ok(Html::new(NewProductTemplate.render().unwrap()))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditProductForm {}

#[put("/api/product/{id}/edit")]
async fn edit_product(
    auth: ProductWriter,
    req: HttpRequest,
    app: web::Data<ServerState>,
    web::Form(form): Form<EditProductForm>,
) -> Result<Html> {
    todo!()
}

#[get("/product/{id}/edit")]
async fn edit_product_page(auth: ProductWriter) -> Result<Html> {
    todo!()
}

#[get("/api/product/store")]
async fn get_products(auth: ProductWriter, app: web::Data<ServerState>) -> Result<Html> {
    todo!()
}

#[get("/product/store")]
async fn get_products_page(auth: ProductWriter, app: web::Data<ServerState>) -> Result<Html> {
    todo!()
}

#[delete("/api/product/{id}")]
async fn delete_product(auth: ProductWriter, app: web::Data<ServerState>) -> Result<Html> {
    todo!()
}

#[get("/api/product/{id}/qr")]
async fn get_product_qr(auth: ProductWriter, app: web::Data<ServerState>) -> Result<Html> {
    todo!()
}

#[get("/product/{id}/qr")]
async fn get_product_qr_page(auth: ProductWriter, app: web::Data<ServerState>) -> Result<Html> {
    todo!()
}
