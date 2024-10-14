// ContactList - CTCL 2023-2024
// File: main.rs
// Purpose: 
// Created: October 9, 2024
// Modified: October 12, 2024



#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "static/"))
            //.app_data(web::Data::new(lysine))
            //.app_data(web::Data::new(sitecfg))
            //.app_data(web::Data::new(memclient))
            .wrap(from_fn(middleware))
    })
    // TODO: define this in the config file
    .bind(("127.0.0.1", "8000"))
    .workers(cpus)
    .run()
    .await
}