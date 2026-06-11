// constantes K
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// rotate right: bits que saem pela direita voltam pela esquerda
fn rotr(x: u32, n: u32) -> u32 { (x << (32 - n)) | (x >> n) }

// shift right: bits que saem pela direita são descartados
fn shr(x: u32, n: u32) -> u32 { x >> n }

// σ0 — usada no message schedule
fn sig0(x: u32) -> u32 { rotr(x, 7) ^ rotr(x, 18) ^ shr(x, 3) }

// σ1 — usada no message schedule
fn sig1(x: u32) -> u32 { rotr(x, 17) ^ rotr(x, 19) ^ shr(x, 10) }

// Σ0 — usada na compressão (grupo a/b/c/d)
fn bsig0(x: u32) -> u32 { rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22) }

// Σ1 — usada na compressão (grupo e/f/g/h)
fn bsig1(x: u32) -> u32 { rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25) }

// choose: para cada bit, e escolhe entre f (e=1) e g (e=0)
fn ch(e: u32, f: u32, g: u32) -> u32 { (e & f) ^ ((!e) & g) }

// majority: para cada bit, resultado é 1 se maioria de a,b,c for 1
fn maj(a: u32, b: u32, c: u32) -> u32 { (a & b) ^ (a & c) ^ (b & c) }

// função principal sha256
pub fn sha256(message: &[u8]) -> [u8; 32]{
    // registradores iniciais — raízes quadradas dos 8 primeiros primos
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    // padding
    let mut resultado: Vec<u8> = Vec::new();
    let len_message = (message.len() as u64) * 8; // tamanho em bits

    resultado.extend_from_slice(message);         // mensagem original
    resultado.push(0x80);                         // bit '1' obrigatório

    let zeros = (120 - (resultado.len() % 64)) % 64;

    resultado.extend(vec![0u8; zeros]);            // zeros de preenchimento
    resultado.extend_from_slice(&len_message.to_be_bytes()); // tamanho em 64 bits big-endian

    // processa cada bloco de 512 bits (64 bytes)
    for bloco in resultado.chunks(64){
        let mut w: [u32; 64] = [0u32; 64];

        // W[0..15] — 16 primeiras palavras direto do bloco (4 bytes cada)
        for i in 0..16{
            w[i] = u32::from_be_bytes([bloco[i*4], bloco[i*4+1], bloco[i*4+2], bloco[i*4+3]]);
        }

        // aqui ocorre a expansão de w[16] para w[63]
        // wrapping_add descarta o overflow e mantém somente 32 bits, se não fizer isto, o rust acusa panic
        for i in 16..64{
            // W[16..63] -> wi[] = sig1(w[i-2]) + w[i-7] + sig0(w[i-15]) + w[i-16]
            w[i] = sig1(w[i-2]).wrapping_add(w[i-7]).wrapping_add(sig0(w[i-15])).wrapping_add(w[i-16]);
        }

        // compressão
        let mut vars = h; // jeito fácil de copiar um array inteiro em rust

        // loop calculando T1 e T2
        let mut t1: u32;
        let mut t2: u32;

        for i in 0..64{
            let h_temp = vars[7];

            t1 = h_temp.wrapping_add(bsig1(vars[4]))
                .wrapping_add(ch(vars[4], vars[5], vars[6]))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);

            t2 = bsig0(vars[0]).wrapping_add(maj(vars[0], vars[1], vars[2]));

            // atualização das variáveis de trabalho
            vars[7] = vars[6];                      // h = g
            vars[6] = vars[5];                      // g = f
            vars[5] = vars[4];                      // f = e
            vars[4] = vars[3].wrapping_add(t1);     // e = d + T1
            vars[3] = vars[2];                      // d = c
            vars[2] = vars[1];                      // c = b
            vars[1] = vars[0];                      // b = a
            vars[0] = t1.wrapping_add(t2);          // a = T1 + T2
        }
        
        // retornando vars para h
        h[0] = h[0].wrapping_add(vars[0]);  // h[0] = a
        h[1] = h[1].wrapping_add(vars[1]);  // h[1] = b
        h[2] = h[2].wrapping_add(vars[2]);  // h[2] = c
        h[3] = h[3].wrapping_add(vars[3]);  // h[3] = d
        h[4] = h[4].wrapping_add(vars[4]);  // h[4] = e
        h[5] = h[5].wrapping_add(vars[5]);  // h[5] = f
        h[6] = h[6].wrapping_add(vars[6]);  // h[6] = g
        h[7] = h[7].wrapping_add(vars[7]);  // h[7] = h
    }

    // montagem do hash final
    let mut hash_final: [u8; 32] = [0u8; 32];
    
    for i in 0..8{
        let bytes = h[i].to_be_bytes();
        hash_final[i*4] = bytes[0];
        hash_final[i*4+1] = bytes[1];
        hash_final[i*4+2] = bytes[2];
        hash_final[i*4+3] = bytes[3];
    }

    hash_final
}