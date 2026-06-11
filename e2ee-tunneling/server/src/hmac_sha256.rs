use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn hmac_sha256(password: &[u8], message: &[u8]) -> Vec<u8>{
    let mut mac = HmacSha256::new_from_slice(password).unwrap();
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}