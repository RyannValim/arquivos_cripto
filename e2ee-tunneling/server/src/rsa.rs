pub struct PublicKey  { pub n: u64, pub e: u64 }
pub struct PrivateKey { pub n: u64, pub d: u64 }

pub fn mod_pow(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut result = 1u64;
    base %= modulo;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulo;
        }
        exp /= 2;
        base = base * base % modulo;
    }
    result
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn mod_inverse(e: u64, phi: u64) -> u64 {
    let mut old_r = phi as i64;
    let mut r = e as i64;
    let mut old_s = 0i64;
    let mut s = 1i64;
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    if old_s < 0 { (old_s + phi as i64) as u64 } else { old_s as u64 }
}

pub fn gerar_chaves(p: u64, q: u64, e: u64) -> (PublicKey, PrivateKey) {
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    assert_eq!(gcd(e, phi), 1, "e precisa ser coprimo de phi(n)!");
    let d = mod_inverse(e, phi);
    (PublicKey { n, e }, PrivateKey { n, d })
}

pub fn cifrar(m: u64, pub_key: &PublicKey) -> u64 {
    assert!(m < pub_key.n, "Mensagem deve ser menor que n!");
    mod_pow(m, pub_key.e, pub_key.n)
}

pub fn decifrar(c: u64, priv_key: &PrivateKey) -> u64 {
    mod_pow(c, priv_key.d, priv_key.n)
}