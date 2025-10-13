use rs_merkle::Hasher;
use sha2::{Digest, Sha256, digest::FixedOutput};

#[derive(Clone)]
pub struct Sha256Algorithm {}
// Check, kyber.

impl Hasher for Sha256Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(data);
        <[u8; 32]>::from(hasher.finalize_fixed())
    }
}
