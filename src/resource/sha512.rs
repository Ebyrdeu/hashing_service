use sha2::Sha512;
use tide::{Body, Request, Response};

use crate::domain::dto::{AutoSalt, HashedPasswordWithSalt, IsEqual, ManualSalt};
use crate::hash::hash_salt_impl::{compare_hash, with_salt};
use crate::utils::salt::random_salt;

pub async fn sha512_manual_salt(mut req: Request<()>) -> tide::Result {
    async {
        let ManualSalt { password, rounds, salt } = req.body_json().await.map_err(|_e| {
            tide::Error::from_str(400, "Invalid request body")
        })?;

        let hashed_password = with_salt::<Sha512>(password, rounds, salt.clone()).await;

        let response = Response::builder(201)
            .body(Body::from_json(&HashedPasswordWithSalt {
                password: hashed_password,
                salt,
            })?)
            .content_type(tide::http::mime::JSON)
            .build();

        Ok(response)
    }
        .await
}


pub async fn sha512_auto_salt(mut req: Request<()>) -> tide::Result {
    async {
        let AutoSalt { password, rounds } = req.body_json().await.map_err(|_e| {
            tide::Error::from_str(400, "Invalid request body")
        })?;

        let salt = random_salt(Default::default());
        let hashed_password = with_salt::<Sha512>(password, rounds, salt.clone()).await;

        let response = Response::builder(201)
            .body(Body::from_json(&HashedPasswordWithSalt {
                password: hashed_password,
                salt,
            })?)
            .content_type(tide::http::mime::JSON)
            .build();

        Ok(response)
    }
        .await
}


pub async fn sha512_compare(mut req: Request<()>) -> tide::Result {
    async {
        let hashed_password = req.param("hashed-password")?.to_string();
        let ManualSalt { password, rounds, salt } = req.body_json().await.map_err(|_e| {
            tide::Error::from_str(400, "Invalid request body")
        })?;

        let is_equal = compare_hash::<Sha512>(password, hashed_password, rounds, salt).await;

        let response = Response::builder(201)
            .body(Body::from_json(&IsEqual { is_equal })?)
            .content_type(tide::http::mime::JSON)
            .build();

        Ok(response)
    }
        .await
}
