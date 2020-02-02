use actix_files as fs;
use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header};

mod services;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let address = "127.0.0.1:8080";

    println!("Starting web server on {}", address);

    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::new()
                    .allowed_origin("http://localhost:4200")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .service(
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind(address)?
    .run()
    .await
}
