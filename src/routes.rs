// ContactList - CTCL 2023-2024
// File: src/routes.rs
// Purpose: Routes
// Created: March 22, 2024
// Modified: April 2, 2024

use actix_web::{
    web, Error, HttpResponse, Responder, Result,
};
use crate::GlobalCfg;
use contactlist::{estconn, mkcontext, read_file};
use contactlist::models::Contact;
use diesel::prelude::*;

pub async fn index(tmpl: web::Data<tera::Tera>, globalcfg: web::Data<GlobalCfg>) -> Result<impl Responder, Error> {
    use contactlist::schema::contacts::dsl::*;

    let mut ctx = mkcontext("ContactList", globalcfg.get_ref().to_owned()).unwrap();

    let dbcon = &mut estconn(&globalcfg.sqlitedburl);
    let entries: Vec<Contact> = contacts.load(dbcon).expect("Error loading contacts");

    ctx.insert("data", &entries);

    ctx.insert("fa", &true);
    ctx.insert("jq", &true);
    ctx.insert("ts", &true);
    
    let s = match tmpl.render("main.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}