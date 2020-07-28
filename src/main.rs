use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/images", "./static/images").show_files_listing())
            .service(Files::new("/", "./static/root").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
