use actix_web::{middleware, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use awc::Client;
use lettre::{
    message::header::ContentType,
    message::Mailbox,
    transport::smtp::authentication::Credentials,
    transport::smtp::{response::Response as SmtpResponse, Error as SmtpError},
    Message as SmtpMessage, SmtpTransport, Transport,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct Message {
    name: String,
    email: String,
    message: String,
    #[serde(rename = "h-captcha-response")]
    h_captcha: String,
}

#[derive(Serialize)]
struct HCaptchaPayload {
    response: String,
    secret: String,
}

#[derive(Deserialize)]
struct HCaptchaResponse {
    success: bool,
    // #[serde(default, rename = "error-codes")]
    // error_codes: Vec<String>,
}

struct EmailInformation {
    address: String,
    username: String,
    password: String,
}

async fn is_captcha_valid(payload: &HCaptchaPayload) -> Result<bool, HttpResponseBuilder> {
    let client = Client::default();
    let response = client
        .post("https://api.hcaptcha.com/siteverify")
        .send_form(payload)
        .await;
    let response = match response {
        Ok(mut content) => content.json::<HCaptchaResponse>().await,
        _ => return Err(HttpResponse::Unauthorized()),
    };

    if let Ok(h_captcha_response) = response {
        Ok(h_captcha_response.success)
    } else {
        Err(HttpResponse::InternalServerError())
    }
}

fn get_email_information() -> Option<EmailInformation> {
    match (
        env::var("EMAIL_ADDRESS"),
        env::var("EMAIL_USERNAME"),
        env::var("EMAIL_PASSWORD"),
    ) {
        (Ok(address), Ok(username), Ok(password)) => Some(EmailInformation {
            address,
            username,
            password,
        }),
        _ => None,
    }
}
async fn send_email(
    message: &Message,
    email_information: EmailInformation,
) -> Result<SmtpResponse, SmtpError> {
    let email = SmtpMessage::builder()
        .to(email_information.address.parse().unwrap())
        .from(Mailbox::new(
            Some(message.name.clone()),
            email_information.username.parse().unwrap(),
        ))
        .subject(format!("Email from `{}`", message.email))
        .header(ContentType::TEXT_PLAIN)
        .body(message.message.clone())
        .expect("Parsing failed");

    let creds = Credentials::new(email_information.username, email_information.password);

    let mailer = SmtpTransport::relay("smtp.eu.mailgun.org")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email)
}

async fn contact(info: web::Json<Message>) -> impl Responder {
    let h_captcha_secret = match env::var("H_CAPTCHA_SECRET") {
        Ok(secret) => secret,
        _ => return HttpResponse::InternalServerError(),
    };
    let h_captcha_payload = HCaptchaPayload {
        response: info.h_captcha.clone(),
        secret: h_captcha_secret,
    };
    let captcha_is_valid = match is_captcha_valid(&h_captcha_payload).await {
        Ok(is_valid) => is_valid,
        Err(error) => return error,
    };

    if !captcha_is_valid {
        return HttpResponse::Unauthorized();
    }
    let email_information = match get_email_information() {
        Some(info) => info,
        _ => return HttpResponse::InternalServerError(),
    };
    let result = send_email(&info, email_information).await;

    if result.is_ok() {
        HttpResponse::Created()
    } else {
        HttpResponse::Unauthorized()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .route("/contact", web::post().to(contact))
    })
    .bind(addr)?
    .run()
    .await
}
