mod handlers;
mod templates;
mod send_mail;

use std::{
    net::SocketAddr,
    time::Duration,
};
use axum::{
    routing::{get, post},
    Router,
};
use axum::http::Method;
use tower_http::{
    services::ServeDir,
    cors::{Any, CorsLayer}
};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a CORS middleware using tower-http's CorsLayer
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .allow_credentials(false)
        .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .route("/",         get(handlers::inject_home))
        .route("/products", get(handlers::inject_products))
        .route("/locations",get(handlers::inject_locations))
        .route("/contact",  get(handlers::inject_contact))

        .route("/submit_form", post(send_mail::form_handler))

        .nest_service("/static", ServeDir::new("../www/static"))

        .layer(cors_layer);

    println!("Server listening on http://{}", addr);


    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}

