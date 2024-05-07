use std::env;
use std::sync::Arc;

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
    let shared_tracer = Arc::new(tracer);

    let address = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").expect("PORT is required");

    log::start();
    let mut app = tide::new();

    app.with(log::LogMiddleware::new());

    let st = shared_tracer.clone();
    app.at("/sha256/method/manual").post(move |req| {
        let mut tracer = st.clone();
        async move {
            sha256_manual_salt(req, &mut tracer).await
        }
    });

    let st = shared_tracer.clone();
    app.at("/sha256/method/auto").post(move |req| {
        let mut tracer = st.clone();
        async move {
            sha256_auto_salt(req, &mut tracer).await
        }
    });

    let st = shared_tracer.clone();
    app.at("/sha256/:hashed-password").post(move |req| {
        let mut tracer = st.clone();
        async move {
            sha256_compare(req, &mut tracer).await
        }
    });


    app.listen(format!("{}:{}", address, port)).await?;
    Ok(())
}

