use actix_files::Files;
use actix_web::{client, web, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslConector, SslMethod};

async fn api_rae(req: HttpRequest) -> impl Responder {
    // Create request builder and send request
    let name = req.match_info().get("word").unwrap_or("World");
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = client::Client::build()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();
    let response = client
        .get("https://dle.rae.es/".to_owned() + name)
        .header("User-Agent", "actix-web/3.0")
        .send() // <- Send request
        .await; // <- Wait for response
    let foo = response.unwrap().body().await.unwrap();
    println!(
        "{:?}",
        rae_rust::search(name, std::str::from_utf8(&foo).unwrap())
    );
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    println!("Running on http://{}", &addr);
    HttpServer::new(|| {
        App::new()
            .route("/rae/v1/{word}", web::get().to(api_rae))
            .service(Files::new("/images", "./static/images").show_files_listing())
            .service(Files::new("/", "./static/root").index_file("index.html"))
    })
    .bind(addr)?
    .run()
    .await
}
