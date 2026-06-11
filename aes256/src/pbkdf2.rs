use crate::hmac_sha256::hmac_sha256;

pub fn pbkdf2(password: &[u8], salt: &[u8], c: u32, dklen: u32) -> Vec<u8>{
    
    let i: u32 = 1;
    let mut message = Vec::new();
    message.extend_from_slice(salt);
    message.extend_from_slice(&i.to_be_bytes());

    let mut u_atual: Vec<u8> = hmac_sha256(password, &message);
    let mut dk: Vec<u8> = u_atual.clone();

    for _ in 2..=c{
        u_atual = hmac_sha256(password, &u_atual);
        for j in 0..32{
            dk[j] = dk[j] ^ u_atual[j];
        }
    }
  
    dk.truncate(dklen as usize);
    dk
}