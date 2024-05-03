// ContactList - CTCL 2023-2024
// File: src/main.rs
// Purpose: Main code
// Created: March 22, 2024
// Modified: April 30, 2024

use actix_files as fs;
use actix_web::{
    middleware, web, App, HttpServer
};
use std::error::Error;
use diesel::sqlite;
use tera::Tera;
use contactlist::{GlobalCfg, read_file, estconn};
mod routes;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let globalcfg: GlobalCfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();
    
    let bindip = globalcfg.bindip;
    let bindport = globalcfg.bindport;
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();
        let globalcfg: GlobalCfg = serde_json::from_str(&read_file("config/config.json").unwrap()).unwrap();

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(globalcfg))
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Always))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/new/").route(web::get().to(new)))
            
    })
    .bind((bindip, bindport))?
    .run()
    .await
}