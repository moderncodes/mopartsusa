use hyper::{client::Client, Body,};
use hyper_tls::HttpsConnector;
use axum::{
    Form,
    http::{Request, StatusCode},
    Json,
    response::IntoResponse,
};
use lettre::{
    Message,
    message::header::ContentType,
    SmtpTransport,
    Transport,
    transport::smtp::authentication::Credentials,
};

use serde::{Serialize,Deserialize,};
use urlencoding::encode;
use std::env;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
    phone: String,
    purpose: String,
    message: String,
    g_recaptcha_response: String,
}

#[derive(Deserialize)]
struct RecaptchaVerificationResponse {success: bool,}

#[derive(Serialize)]
pub struct JsonResponse {
    status: String,
    message: String,
}

pub async fn verify_recaptcha(token: &str, secret_key: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let https_connector = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https_connector);

    let request_body = format!(
        "secret={}&response={}",
        encode(secret_key),
        encode(token)
    );

    let request = Request::builder()
        .method("POST")
        .uri("https://www.google.com/recaptcha/api/siteverify")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(request_body))
        .unwrap();

    let response = client.request(request).await?;

    if response.status() == StatusCode::OK {
        let bytes = hyper::body::to_bytes(response.into_body()).await?;
        let verification_response: RecaptchaVerificationResponse = serde_json::from_slice(&bytes)?;
        Ok(verification_response.success)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to verify reCAPTCHA",
        )))
    }
}

pub async fn form_handler(Form(form_data): Form<FormData>) -> impl IntoResponse {
    let secret_key = env::var("RECAPTCHA_SECRET").expect("RECAPTCHA_SECRET not set");
    let is_valid = verify_recaptcha(&form_data.g_recaptcha_response, &secret_key).await.unwrap_or(false);

    if is_valid {
        let email = Message::builder()
            .from(form_data.email.parse().unwrap())
            .reply_to(form_data.email.parse().unwrap())
            .to("genef@mopartsusa.com".parse().unwrap())
            .subject("New contact form submission")
            .header(ContentType::TEXT_PLAIN)
            .body(format!(
                "Name: {}\nEmail: {}\nPhone: {}\nPurpose: {}\nMessage:\n{}",
                form_data.name,
                form_data.email,
                form_data.phone,
                form_data.purpose,
                form_data.message
            ))
            .unwrap();

        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
        let creds = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::relay("smtpout.secureserver.net")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => Json(JsonResponse {
                status: "success".to_string(),
                message: "Form submitted successfully!".to_string(),
            }),
            Err(e) => {
                eprintln!("Failed to send email: {:?}", e);
                Json(JsonResponse {
                    status: "error".to_string(),
                    message: "Failed to send email".to_string(),
                })
            }
        }
    } else {
        Json(JsonResponse {
            status: "error".to_string(),
            message: "Invalid reCAPTCHA!".to_string(),
        })
    }
}