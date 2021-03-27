use actix_web::{
    client::{Client, ClientResponse, Connector},
    dev::{Decompress, Payload},
    middleware, web, App, Error as ActixError, HttpRequest, HttpServer, Responder,
};
use openssl::ssl::{SslConnector, SslMethod};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::collections::BTreeMap;

async fn api_rae(req: HttpRequest) -> impl Responder {
    /// https://url.spec.whatwg.org/#fragment-percent-encode-set
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

    let name = req.match_info().get("word").unwrap();

    // https://en.wikipedia.org/wiki/Percent-encoding
    let name: &str = &utf8_percent_encode(name, FRAGMENT).to_string();
    let ssl_connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    // Create request builder and send request
    let client = Client::builder()
        .connector(Connector::new().ssl(ssl_connector).finish())
        .finish();

    let mut response = make_request_rae(name, &client).await;

    if response.status().as_u16() == 301 {
        response = make_request_rae(
            &utf8_percent_encode(
                response
                    .headers()
                    .get("location")
                    .and_then(|location| std::str::from_utf8(location.as_bytes()).ok())
                    .unwrap(),
                FRAGMENT,
            )
            .to_string()[1..],
            &client,
        )
        .await;
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
            let mut response =
                make_request_rae(&utf8_percent_encode(&word, FRAGMENT).to_string(), &client).await;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .route("/rae/v1/{word}", web::get().to(api_rae))
    })
    // Create request builder and send request
    .bind(addr)?
    .run()
    .await
}
