use std::env;

use tide::log;

mod resource;
mod domain;
mod hash;
mod utils;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let address = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").expect("PORT is required");

    log::start();

    let mut app = tide::new();
    app.with(log::LogMiddleware::new());

    app.at("/sha256/method/manual").post(resource::sha256::sha256_manual_salt);
    app.at("/sha256/method/auto").post(resource::sha256::sha256_auto_salt);
    app.at("/sha256/:hashed-password").post(resource::sha256::sha256_compare);

    app.at("/sha512/method/manual").post(resource::sha512::sha512_manual_salt);
    app.at("/sha512/method/auto").post(resource::sha512::sha512_auto_salt);
    app.at("/sha512/:hashed-password").post(resource::sha512::sha512_compare);

    app.listen(format!("{}:{}", address, port)).await?;
    Ok(())
}