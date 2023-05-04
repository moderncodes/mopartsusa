mod handlers;
mod templates;

use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",         get(handlers::inject_home))
        .route("/products", get(handlers::inject_products))
        .route("/locations",get(handlers::inject_locations))
        .route("/contact",  get(handlers::inject_contact))

        .nest_service("/static", ServeDir::new("../www/static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}

