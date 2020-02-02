use actix_files as fs;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web, http::header};

#[cfg(test)]
mod tests;

mod model;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let address = "localhost:8080";

    println!("Starting web server on {}", address);

    HttpServer::new(move || {
        App::new()
            .wrap(
            Cors::new()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_header(header::CONTENT_TYPE)
                    .send_wildcard()
                    .max_age(3600)
                    .finish(),
            )
            .route("/user", web::get().to(crate::services::get_users))
            .service(
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind(address)?
    .run()
    .await
}
