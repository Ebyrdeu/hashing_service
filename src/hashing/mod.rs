pub mod dto {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct HashedPassword {
        pub password: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct HashedPasswordWithSalt {
        pub password: String,
        pub salt: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct IsEqual {
        pub is_equal: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct ManualSalt {
        pub password: String,
        pub rounds: usize,
        pub salt: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct AutoSalt {
        pub password: String,
        pub rounds: usize,
    }
}

pub mod hashing {
    use sha2::{Digest, Sha256, Sha512};

    pub(crate) trait Hasher {
        fn update(&mut self, data: &[u8]);
        fn finalize_reset(&mut self) -> Vec<u8>;
        fn finalize(self) -> Vec<u8>;
    }

    impl Hasher for Sha256 {
        fn update(&mut self, data: &[u8]) {
            Digest::update(self, data);
        }

        fn finalize_reset(&mut self) -> Vec<u8> {
            Digest::finalize_reset(self).to_vec()
        }

        fn finalize(self) -> Vec<u8> {
            Digest::finalize(self).to_vec()
        }
    }

    impl Hasher for Sha512 {
        fn update(&mut self, data: &[u8]) {
            Digest::update(self, data);
        }

        fn finalize_reset(&mut self) -> Vec<u8> {
            Digest::finalize_reset(self).to_vec()
        }

        fn finalize(self) -> Vec<u8> {
            Digest::finalize(self).to_vec()
        }
    }

    pub async fn with_salt<D: Hasher + Default>(
        password: String,
        rounds: usize,
        salt: String,
    ) -> String {
        let mut password_with_salt: String = String::from(password);
        password_with_salt.push_str(salt.as_str());

        let mut hasher = D::default();
        hasher.update(password_with_salt.as_bytes());

        for _ in 0..rounds {
            let fr = hasher.finalize_reset();
            hasher.update(&fr);
        }

        let result = hasher.finalize();

        result
            .iter()
            .map(|bytes| format!("{:02x}", bytes))
            .collect::<String>()
    }

    pub async fn compare_hash<D: Hasher + Default>(
        plain_password: String,
        hashed_password: String,
        rounds: usize,
        salt: String,
    ) -> bool {
        let hashed_plain_password = with_salt::<D>(plain_password, rounds, salt).await;

        hashed_password == hashed_plain_password
    }
}
