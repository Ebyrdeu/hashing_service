use sha2::{Digest, Sha256};

pub trait Hasher {
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
