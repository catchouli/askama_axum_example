use std::{error::Error, net::SocketAddr};

use askama::Template;
use axum::{
    routing::get,
    Router,
};
//use tower::util::ServiceExt;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "test" }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    log::info!("Hello world");

    let app = Router::new()
        .route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
