use opentelemetry::global;
use opentelemetry::trace::{Span, Tracer};
use sha2::Sha512;
use tide::{Body, Request, Response};

use crate::domain::dto::{AutoSalt, HashedPassword, HashedPasswordWithSalt, IsEqual, ManualSalt};
use crate::hash::hash_salt_impl::{compare_hash, with_salt};
use crate::utils::salt::random_salt;

pub async fn sha512_manual_salt(mut req: Request<()>) -> tide::Result {
    let tracer = global::tracer("app_tracer");
    let mut span = tracer.start("sha512_manual_salt");

    let ManualSalt { password, rounds, salt } = req.body_json().await?;

    let hashed_password = with_salt::<Sha512>(password, rounds, salt).await;

    let response = Response::builder(201)
        .body(Body::from_json(&HashedPassword {
            password: hashed_password,
        })?)
        .content_type(tide::http::mime::JSON)
        .build();

    span.end();
    Ok(response)
}

pub async fn sha512_auto_salt(mut req: Request<()>) -> tide::Result {
    let tracer = global::tracer("app_tracer");
    let mut span = tracer.start("sha512_auto_salt");

    let AutoSalt { password, rounds } = req.body_json().await?;

    let salt = random_salt(Default::default());
    let hashed_password = with_salt::<Sha512>(password, rounds, salt.clone()).await;

    let response = Response::builder(201)
        .body(Body::from_json(&HashedPasswordWithSalt {
            password: hashed_password,
            salt,
        })?)
        .content_type(tide::http::mime::JSON)
        .build();

    span.end();
    Ok(response)
}

pub async fn sha512_compare(mut req: Request<()>) -> tide::Result {
    let tracer = global::tracer("app_tracer");
    let mut span = tracer.start("sha512_compare");

    let hashed_password = req.param("hashed-password")?;
    let hashed_password = hashed_password.to_string();

    let ManualSalt { password, rounds, salt } = req.body_json().await?;

    let is_equal = compare_hash::<Sha512>(password, hashed_password, rounds, salt).await;

    let response = Response::builder(201)
        .body(Body::from_json(&IsEqual { is_equal })?)
        .content_type(tide::http::mime::JSON)
        .build();

    span.end();
    Ok(response)
}
