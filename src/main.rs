use actix_files::{Files, NamedFile};
use actix_web::{client::{Connector, Client}, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Error as ActixError};
use openssl::ssl::{SslConnector, SslMethod};

async fn api_rae(req: HttpRequest) -> impl Responder {
    // Create request builder and send request
    let name = req.match_info().get("word").unwrap_or("World");
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::build()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let mut response = client
        .get("https://dle.rae.es/".to_owned() + name)
        .send() // <- Send request
        .await  // <- Wait for response
        .unwrap();

    if response.status().as_u16() == 301 {    
        response = client
            .get("https://dle.rae.es".to_owned() + response.headers().get("location").unwrap().to_str().unwrap())
            .send() // <- Send request
            .await // <- Wait for response
            .unwrap();     
    }
    let body = response.body().await.unwrap();
    let map = rae_rust::search(name, std::str::from_utf8(&body).expect("Failed to parse body"))
        .expect("Failed to obtain the dicctionary of meanings");

    Ok::<_, ActixError>(web::Json(map))
}

fn not_found(r: HttpRequest) -> HttpResponse {
    NamedFile::open("./static/root/html/notfound.html".parse::<std::path::PathBuf>().expect("Failed to parse path")).unwrap().set_status_code("404".parse().unwrap()).into_response(&r).unwrap()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
println!("{:?}", std::fs::canonicalize(&std::path::PathBuf::from("./")));
    let addr = "127.0.0.1:8080";
    println!("Running on http://{}", &addr);
    HttpServer::new(|| {
        App::new()
            .route("/rae/v1/{word}", web::get().to(api_rae))
            .service(Files::new("/images", "./static/images"))
            .service(Files::new("/", "./static/root").index_file("index.html"))
            .default_service(web::get().to(not_found))
    })
    .bind(addr)?
    .run()
    .await
}
