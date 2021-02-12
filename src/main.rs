use actix_files::{Files, NamedFile};
use actix_web::{
    client::{Client, ClientResponse, Connector},
    dev::{Decompress, Payload},
    web, App, Error as ActixError, HttpRequest, HttpResponse, HttpServer, Responder,
};
use openssl::ssl::{SslConnector, SslMethod};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::collections::BTreeMap;

async fn api_rae(req: HttpRequest) -> impl Responder {
    /// https://url.spec.whatwg.org/#fragment-percent-encode-set
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

    // Create request builder and send request
    let name = req.match_info().get("word").unwrap_or("World");

    // https://en.wikipedia.org/wiki/Percent-encoding
    let name: &str = &utf8_percent_encode(name, FRAGMENT).collect::<String>();
    let ssl_connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let client = Client::builder()
        .connector(Connector::new().ssl(ssl_connector).finish())
        .finish();

    let mut response = make_request_rae(name, &client).await;

    if response.status().as_u16() == 301 {
        response = client
            .get(
                "https://dle.rae.es".to_owned()
                    + &utf8_percent_encode(std::str::from_utf8(response
                        .headers()
                        .get("location")
                        .unwrap()
                        .as_bytes())
                        .unwrap(), FRAGMENT).collect::<String>()
            )
            .send() // <- Send request
            .await // <- Wait for response
            .unwrap();
    }
    let body = response.body().await.unwrap();

    let result_rae = rae_rust::search(std::str::from_utf8(&body).expect("Failed to parse body"));

    let meanings = match result_rae {
        Ok(map) => map,
        Err(rae_rust::WebScrapError::ParseError(_)) => {
            let mut map: BTreeMap<String, rae_rust::ValueVariant> = BTreeMap::new();
            map.insert(
                String::from("Error"),
                rae_rust::ValueVariant::String(String::from("Error processing the request")),
            );
            map
        }
        Err(rae_rust::WebScrapError::Other(word)) => {
            let mut response = make_request_rae(&utf8_percent_encode(&word, FRAGMENT).collect::<String>(), &client).await;
            let body = response.body().await.unwrap();

            rae_rust::search(std::str::from_utf8(&body).expect("Failed to parse body")).unwrap()
        }
        Err(rae_rust::WebScrapError::NotFound) => {
            let mut map: BTreeMap<String, rae_rust::ValueVariant> = BTreeMap::new();
            map.insert(
                String::from("Error"),
                rae_rust::ValueVariant::String(String::from("Word not found")),
            );
            map
        }
    };

    Ok::<_, ActixError>(web::Json(meanings))
}

async fn make_request_rae(word: &str, client: &Client) -> ClientResponse<Decompress<Payload>> {
    client
        .get("https://dle.rae.es/".to_owned() + word)
        .send() // <- Send request
        .await // <- Wait for response
        .unwrap()
}

fn not_found(req: HttpRequest) -> HttpResponse {
    let path: std::path::PathBuf = "./static/root/html/notfound.html"
        .parse()
        .expect("Failed to parse path");
    let namedfile = NamedFile::open(path).expect("Failed to create NamedFile");
    namedfile
        .set_status_code("404".parse().unwrap())
        .into_response(&req)
        .expect("Failed to convert to response")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
