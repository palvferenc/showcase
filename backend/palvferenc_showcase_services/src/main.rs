use actix_files as fs;
use actix_web::{App, HttpServer};

mod services;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let address = "127.0.0.1:8080";

    println!("Starting web server on {}", address);

    HttpServer::new(|| {
        App::new()
            .service(
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind(address)?
    .run()
    .await
}
