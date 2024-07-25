// ContactList - CTCL 2023-2024
// File: src/routes.rs
// Purpose: Routes
// Created: March 22, 2024
// Modified: June 14, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use contactlist::Combined;



pub async fn index(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<Combined>) -> Result<impl Responder, Error> {
    let mut ctx = mkcontext("ContactList", globalcfg.get_ref().to_owned()).unwrap();

    ctx.insert("data", &entries);

    ctx.insert("fa", &true);
    ctx.insert("jq", &true);
    ctx.insert("ts", &true);
    ctx.insert("ce", &false);

    let s = match tmpl.render("main.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}

pub async fn new(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<Combined>) -> Result<impl Responder, Error> {

}

pub async fn view(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<Combined>) -> Result<impl Responder, Error> {

}

pub async fn delete(tmpl: web::Data<tera::Tera>, sitecfg: web::Data<Combined>) -> Result<impl Responder, Error> {
    
}