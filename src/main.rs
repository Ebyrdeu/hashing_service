use tide::log;

use crate::resource::sha256::{sha256_auto_salt, sha256_compare, sha256_manual_salt};
use crate::resource::sha512::{sha512_auto_salt, sha512_compare, sha512_manual_salt};
use crate::tracer::jaeger::init_tracer;

mod resource;
mod domain;
mod hash;
mod utils;
mod tracer;

#[async_std::main]
async fn main() -> tide::Result<()> {
    init_tracer().await.expect("Tracer should be initialized");

    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").expect("PORT is required");

    log::start();
    let mut app = tide::new();

    app.with(log::LogMiddleware::new());

    app.at("/sha256/method/manual").post(sha256_manual_salt);
    app.at("/sha256/method/auto").post(sha256_auto_salt);
    app.at("/sha256/:hashed-password").post(sha256_compare);

    app.at("/sha512/method/manual").post(sha512_manual_salt);
    app.at("/sha512/method/auto").post(sha512_auto_salt);
    app.at("/sha512/:hashed-password").post(sha512_compare);

    app.listen(format!("{}:{}", address, port)).await?;
    Ok(())
}
