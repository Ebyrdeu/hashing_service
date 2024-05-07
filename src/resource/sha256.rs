use std::sync::Arc;

use opentelemetry::KeyValue;
use opentelemetry::trace::{TraceContextExt, Tracer};
use opentelemetry_sdk::trace::Tracer as sdkTracer;
use sha2::Sha256;
use tide::{Body, Request, Response};

use crate::domain::dto::{AutoSalt, HashedPasswordWithSalt, IsEqual, ManualSalt};
use crate::hash::hash_salt_impl::{compare_hash, with_salt};
use crate::utils::salt::random_salt;

pub async fn sha256_manual_salt(mut req: Request<()>, tracer: &Arc<sdkTracer>) -> tide::Result {
    tracer.in_span("sha256_manual_salt", |cx| {
        async move {
            let ManualSalt { password, rounds, salt } = req.body_json().await.map_err(|e| {
                cx.span().add_event("Invalid request body", vec![KeyValue::new("error", e.to_string())]);
                tide::Error::from_str(400, "Invalid request body")
            })?;

            let hashed_password = with_salt::<Sha256>(password, rounds.clone(), salt.clone()).await;
            cx.span().add_event("Password hashed", vec![
                KeyValue::new("hashed_password", hashed_password.clone()),
                KeyValue::new("rounds", rounds.to_string()),
                KeyValue::new("salt", format!("{:?}", salt)),
            ]);

            let response = Response::builder(201)
                .body(Body::from_json(&HashedPasswordWithSalt {
                    password: hashed_password,
                    salt,
                })?)
                .content_type(tide::http::mime::JSON)
                .build();

            Ok(response)
        }
    }).await
}

pub async fn sha256_auto_salt(mut req: Request<()>, tracer: &mut Arc<sdkTracer>) -> tide::Result {
    tracer.in_span("sha256_auto_salt", |cx| {
        async move {
            let AutoSalt { password, rounds } = req.body_json().await.map_err(|e| {
                cx.span().add_event("Invalid request body", vec![KeyValue::new("error", e.to_string())]);
                tide::Error::from_str(400, "Invalid request body")
            })?;

            let salt = random_salt(Default::default());
            cx.span().add_event("Salt generated", vec![KeyValue::new("salt", format!("{:?}", salt))]);

            let hashed_password = with_salt::<Sha256>(password, rounds, salt.clone()).await;
            cx.span().add_event("Password hashed", vec![
                KeyValue::new("hashed_password", hashed_password.clone()),
                KeyValue::new("rounds", rounds.to_string()),
            ]);

            let response = Response::builder(201)
                .body(Body::from_json(&HashedPasswordWithSalt {
                    password: hashed_password,
                    salt,
                })?)
                .content_type(tide::http::mime::JSON)
                .build
                ();
            Ok(response)
        }
    }).await
}

pub async fn sha256_compare(mut req: Request<()>, tracer: &mut Arc<sdkTracer>) -> tide::Result {
    tracer.in_span("sha256_compare", |cx| {
        async move {
            let hashed_password = req.param("hashed-password")?.to_string();

            let ManualSalt { password, rounds, salt } = req.body_json().await.map_err(|e| {
                cx.span().add_event("Invalid request body", vec![KeyValue::new("error", e.to_string())]);
                tide::Error::from_str(400, "Invalid request body")
            })?;

            let is_equal = compare_hash::<Sha256>(password, hashed_password.clone(), rounds.clone(), salt.clone()).await;

            cx.span().add_event("Hash comparison", vec![
                KeyValue::new("result", is_equal.to_string()),
                KeyValue::new("hashed_password", hashed_password),
                KeyValue::new("rounds", rounds.to_string()),
                KeyValue::new("salt", format!("{:?}", salt)),
            ]);

            let response = Response::builder(201)
                .body(Body::from_json(&IsEqual { is_equal })?)
                .content_type(tide::http::mime::JSON)
                .build();

            Ok(response)
        }
    }).await
}