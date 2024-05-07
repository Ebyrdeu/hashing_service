use std::env;

use opentelemetry::trace::Tracer;
use tide::log;

use crate::resource::sha256::{sha256_auto_salt, sha256_compare, sha256_manual_salt};
use crate::tracing::otlp::init_oltp;

mod resource;
mod domain;
mod hash;
mod utils;
mod tracing;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let tracer = init_oltp().unwrap();

    let address = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").expect("PORT is required");

    log::start();
    let mut app = tide::new();

    app.with(log::LogMiddleware::new());

    tracer.in_span("sha256_manual_salt", |_| {
        app.at("/sha256/method/manual").post(|req| sha256_manual_salt(req));
    });

    tracer.in_span("sha256_auto_salt", |_| {
        app.at("/sha256/method/auto").post(|req| sha256_auto_salt(req));
    });

    tracer.in_span("sha256_verify", |_| {
        app.at("/sha256/:hashed-password").post(|req| sha256_compare(req));
    });

    app.listen(format!("{}:{}", address, port)).await?;
    Ok(())
}

