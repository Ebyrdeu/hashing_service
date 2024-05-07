use crate::hash::hasher::Hasher;

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
