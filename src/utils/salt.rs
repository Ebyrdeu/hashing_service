use rand::distributions::Alphanumeric;
use rand::Rng;

pub struct Salt {
   pub chars: usize,
}

impl Default for Salt {
    fn default() -> Self {
        Self {
            chars: 16
        }
    }
}

pub fn random_salt(length: Salt) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(length.chars).map(char::from).collect::<String>()
}
