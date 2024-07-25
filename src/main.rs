// ContactList - CTCL 2023-2024
// File: src/main.rs
// Purpose: Main code
// Created: March 22, 2024
// Modified: June 13, 2024

use actix_files as fs;
use actix_web::{
    middleware, web, App, HttpServer
};
use tera::Tera;
mod routes;
use routes::*;
use contactlist::{get_all_configs, Combined};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let combined: Combined = get_all_configs().unwrap();
    let bindip = combined.config.bindip;
    let bindport = combined.config.bindport;

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();
        let combined: Combined = get_all_configs().unwrap();

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(combined))
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Always))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/new/").route(web::get().to(new)))
            
    })
    .bind((bindip, bindport))?
    .run()
    .await
}