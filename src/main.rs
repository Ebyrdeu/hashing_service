use rand::distributions::Alphanumeric;
use rand::Rng;
use sha2::{Sha256, Sha512};
use tide::{Body, Request, Response};

use crate::dto::{AutoSalt, HashedPassword, HashedPasswordWithSalt, IsEqual, ManualSalt};
use crate::hash_salt_impl::{compare_hash, with_salt};

mod hasher;
mod hash_salt_impl;
mod dto;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").expect("PORT is required");

    let mut app = tide::new();

    app.at("/sha256/method/manual").post(sha256_manual_salt);
    app.at("/sha256/method/auto").post(sha256_auto_salt);
    app.at("/sha256/:hashed-password").post(sha256_compare);

    app.at("/sha512/method/manual").post(sha512_manual_salt);
    app.at("/sha512/method/auto").post(sha512_auto_salt);
    app.at("/sha512/:hashed-password").post(sha512_compare);

    app.listen(format!("{}:{}", address, port)).await?;
    Ok(())
}

async fn sha256_manual_salt(mut req: Request<()>) -> tide::Result {
    let ManualSalt {
        password,
        rounds,
        salt,
    } = req.body_json().await?;
    let hashed_password = with_salt::<Sha256>(password, rounds, salt).await;

    Ok(Response::builder(201)
        .body(Body::from_json(&HashedPassword {
            password: hashed_password,
        })?)
        .content_type(tide::http::mime::JSON)
        .build())
}

async fn sha256_auto_salt(mut req: Request<()>) -> tide::Result {
    let AutoSalt { password, rounds } = req.body_json().await?;

    let salt = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>();

    let hashed_password = with_salt::<Sha256>(password, rounds, salt.clone()).await;

    Ok(Response::builder(201)
        .body(Body::from_json(&HashedPasswordWithSalt {
            password: hashed_password,
            salt,
        })?)
        .content_type(tide::http::mime::JSON)
        .build())
}

async fn sha256_compare(mut req: Request<()>) -> tide::Result {
    let hashed_password = req.param("hashed-password")?;
    let hashed_password = hashed_password.to_string();

    let ManualSalt {
        password,
        rounds,
        salt,
    } = req.body_json().await?;

    let is_equal = compare_hash::<Sha256>(password, hashed_password, rounds, salt).await;

    Ok(Response::builder(201)
        .body(Body::from_json(&IsEqual { is_equal })?)
        .content_type(tide::http::mime::JSON)
        .build())
}

async fn sha512_manual_salt(mut req: Request<()>) -> tide::Result {
    let ManualSalt {
        password,
        rounds,
        salt,
    } = req.body_json().await?;
    let hashed_password = with_salt::<Sha512>(password, rounds, salt).await;

    Ok(Response::builder(201)
        .body(Body::from_json(&HashedPassword {
            password: hashed_password,
        })?)
        .content_type(tide::http::mime::JSON)
        .build())
}

async fn sha512_auto_salt(mut req: Request<()>) -> tide::Result {
    let AutoSalt { password, rounds } = req.body_json().await?;

    let salt = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>();

    let hashed_password = with_salt::<Sha512>(password, rounds, salt.clone()).await;

    Ok(Response::builder(201)
        .body(Body::from_json(&HashedPasswordWithSalt {
            password: hashed_password,
            salt,
        })?)
        .content_type(tide::http::mime::JSON)
        .build())
}

async fn sha512_compare(mut req: Request<()>) -> tide::Result {
    let hashed_password = req.param("hashed-password")?;
    let hashed_password = hashed_password.to_string();

    let ManualSalt {
        password,
        rounds,
        salt,
    } = req.body_json().await?;

    let is_equal = compare_hash::<Sha512>(password, hashed_password, rounds, salt).await;

    Ok(Response::builder(201)
        .body(Body::from_json(&IsEqual { is_equal })?)
        .content_type(tide::http::mime::JSON)
        .build())
}
