use rand::rngs::OsRng;
use rand::RngCore;

const SBOX: [u8; 256] = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
];

const INV_SBOX: [u8; 256] = [
    0x52,0x09,0x6a,0xd5,0x30,0x36,0xa5,0x38,0xbf,0x40,0xa3,0x9e,0x81,0xf3,0xd7,0xfb,
    0x7c,0xe3,0x39,0x82,0x9b,0x2f,0xff,0x87,0x34,0x8e,0x43,0x44,0xc4,0xde,0xe9,0xcb,
    0x54,0x7b,0x94,0x32,0xa6,0xc2,0x23,0x3d,0xee,0x4c,0x95,0x0b,0x42,0xfa,0xc3,0x4e,
    0x08,0x2e,0xa1,0x66,0x28,0xd9,0x24,0xb2,0x76,0x5b,0xa2,0x49,0x6d,0x8b,0xd1,0x25,
    0x72,0xf8,0xf6,0x64,0x86,0x68,0x98,0x16,0xd4,0xa4,0x5c,0xcc,0x5d,0x65,0xb6,0x92,
    0x6c,0x70,0x48,0x50,0xfd,0xed,0xb9,0xda,0x5e,0x15,0x46,0x57,0xa7,0x8d,0x9d,0x84,
    0x90,0xd8,0xab,0x00,0x8c,0xbc,0xd3,0x0a,0xf7,0xe4,0x58,0x05,0xb8,0xb3,0x45,0x06,
    0xd0,0x2c,0x1e,0x8f,0xca,0x3f,0x0f,0x02,0xc1,0xaf,0xbd,0x03,0x01,0x13,0x8a,0x6b,
    0x3a,0x91,0x11,0x41,0x4f,0x67,0xdc,0xea,0x97,0xf2,0xcf,0xce,0xf0,0xb4,0xe6,0x73,
    0x96,0xac,0x74,0x22,0xe7,0xad,0x35,0x85,0xe2,0xf9,0x37,0xe8,0x1c,0x75,0xdf,0x6e,
    0x47,0xf1,0x1a,0x71,0x1d,0x29,0xc5,0x89,0x6f,0xb7,0x62,0x0e,0xaa,0x18,0xbe,0x1b,
    0xfc,0x56,0x3e,0x4b,0xc6,0xd2,0x79,0x20,0x9a,0xdb,0xc0,0xfe,0x78,0xcd,0x5a,0xf4,
    0x1f,0xdd,0xa8,0x33,0x88,0x07,0xc7,0x31,0xb1,0x12,0x10,0x59,0x27,0x80,0xec,0x5f,
    0x60,0x51,0x7f,0xa9,0x19,0xb5,0x4a,0x0d,0x2d,0xe5,0x7a,0x9f,0x93,0xc9,0x9c,0xef,
    0xa0,0xe0,0x3b,0x4d,0xae,0x2a,0xf5,0xb0,0xc8,0xeb,0xbb,0x3c,0x83,0x53,0x99,0x61,
    0x17,0x2b,0x04,0x7e,0xba,0x77,0xd6,0x26,0xe1,0x69,0x14,0x63,0x55,0x21,0x0c,0x7d,
];

const RCON: [u8; 15] = [
    0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d
];

fn sub_word(w: [u8; 4]) -> [u8; 4]{
    let mut result: [u8; 4] = [0u8; 4];

    result[0] = SBOX[w[0] as usize];
    result[1] = SBOX[w[1] as usize];
    result[2] = SBOX[w[2] as usize];
    result[3] = SBOX[w[3] as usize];

    result
}

fn rotate_word(w: [u8; 4]) -> [u8; 4]{
    let mut result: [u8; 4] = [0u8; 4];
    result[0] = w[1];
    result[1] = w[2];
    result[2] = w[3];
    result[3] = w[0];

    result
}

fn schedule_core(w: [u8; 4], i: usize) -> [u8; 4]{
    let mut result: [u8; 4] = sub_word(rotate_word(w));
    result[0] ^= RCON[i];
    
    result
}

fn xor_words(a: & [u8; 4], b: & [u8; 4]) -> [u8; 4]{
    let mut result: [u8; 4] = [0u8; 4];
    result[0] = a[0] ^ b[0];
    result[1] = a[1] ^ b[1];
    result[2] = a[2] ^ b[2];
    result[3] = a[3] ^ b[3];

    result
}

pub fn key_expansion(key: &[u8], dklen: u32) -> Vec<[u8; 16]> {
    let nk = (dklen / 4) as usize;
    let num_rounds = (dklen / 4) + 6;
    let total_palavras = ((num_rounds + 1) * 4) as usize;

    let mut palavras: Vec<[u8; 4]> = key.chunks(4)
        .map(|c| c.try_into().unwrap())
        .collect();
    
    for i in nk..total_palavras {
        let w_anterior = palavras[i - 1];

        let nova_palavra = if i % nk == 0 {
            xor_words(&palavras[i - nk], &schedule_core(w_anterior, i / nk))
        } else if nk == 8 && i % nk == 4 {
            xor_words(&palavras[i - nk], &sub_word(w_anterior))
        } else {
            xor_words(&palavras[i - nk], &w_anterior)
        };

        palavras.push(nova_palavra);
    }

    let mut subchaves: Vec<[u8; 16]> = Vec::new();
    for bloco in palavras.chunks(4) {
        let mut subchave = [0u8; 16];
        subchave[0..4].copy_from_slice(&bloco[0]);
        subchave[4..8].copy_from_slice(&bloco[1]);
        subchave[8..12].copy_from_slice(&bloco[2]);
        subchave[12..16].copy_from_slice(&bloco[3]);
        subchaves.push(subchave);
    }

    subchaves
}

pub fn bytes_to_state(plaintext: [u8; 16]) -> [[u8; 4]; 4]{
    let mut state: [[u8; 4]; 4] = [[0u8; 4]; 4];
    let mut i_ptxt = 0;

    for col in 0..4{
        for lin in 0..4{
            state[col][lin] = plaintext[i_ptxt];
            i_ptxt += 1;
        }
    }

    state
}

pub fn add_round_key(state: [[u8; 4]; 4], subkey: &[u8; 16]) -> [[u8; 4]; 4]{
    let mut state = state;
    let mut i_sb = 0;
    for col in 0..4{
        for lin in 0..4{
            state[col][lin] = state[col][lin] ^ subkey[i_sb];
            i_sb += 1;
        }
    }
    
    state
}

pub fn sub_bytes(state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    let mut state = state;
    
    for col in 0..4{
        for lin in 0..4{
            state[col][lin] = SBOX[state[col][lin] as usize];
        }        
    }

    state
}

pub fn inv_sub_bytes(state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    let mut state = state;
    
    for col in 0..4{
        for lin in 0..4{
            state[col][lin] = INV_SBOX[state[col][lin] as usize];
        }
    }

    state
}

pub fn shift_rows(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut state = state;

    for lin in 0..4 {
        let mut temp_array: [u8; 4] = [
            state[0][lin],
            state[1][lin],
            state[2][lin],
            state[3][lin],
        ];

        temp_array.rotate_left(lin);

        state[0][lin] = temp_array[0];
        state[1][lin] = temp_array[1];
        state[2][lin] = temp_array[2];
        state[3][lin] = temp_array[3];
    }

    state
}

pub fn inv_shift_rows(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut state = state;

    for lin in 0..4 {
        let mut temp_array: [u8; 4] = [
            state[0][lin],
            state[1][lin],
            state[2][lin],
            state[3][lin],
        ];

        temp_array.rotate_right(lin);

        state[0][lin] = temp_array[0];
        state[1][lin] = temp_array[1];
        state[2][lin] = temp_array[2];
        state[3][lin] = temp_array[3];
    }

    state
}

fn gf_mul2(x: u8) -> u8 {
    if x & 0x80 != 0 {
        (x << 1) ^ 0x1b
    } else {
        x << 1
    }
}

fn gf_mul3(x: u8) -> u8 {
    gf_mul2(x) ^ x
}

fn gf_mul9(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x))) ^ x
}

fn gf_mul11(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x))) ^ gf_mul2(x) ^ x
}

fn gf_mul13(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x))) ^ gf_mul2(gf_mul2(x)) ^ x
}

fn gf_mul14(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x))) ^ gf_mul2(gf_mul2(x)) ^ gf_mul2(x)
}

pub fn mix_columns(state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    let mut novo_state = [[0u8; 4]; 4];

    for col in 0..4{
        let col_atual = [
            state[col][0],
            state[col][1],
            state[col][2],
            state[col][3]
        ];

        novo_state[col][0] = gf_mul2(col_atual[0]) ^ gf_mul3(col_atual[1]) ^ col_atual[2] ^ col_atual[3];
        novo_state[col][1] = col_atual[0] ^ gf_mul2(col_atual[1]) ^ gf_mul3(col_atual[2]) ^ col_atual[3];
        novo_state[col][2] = col_atual[0] ^ col_atual[1] ^ gf_mul2(col_atual[2]) ^ gf_mul3(col_atual[3]);
        novo_state[col][3] = gf_mul3(col_atual[0]) ^ col_atual[1] ^ col_atual[2] ^ gf_mul2(col_atual[3]);
    }

    novo_state
}

pub fn inv_mix_columns(state: [[u8; 4]; 4]) -> [[u8; 4]; 4]{
    let mut novo_state = [[0u8; 4]; 4];

    for col in 0..4{
        let col_atual = [
            state[col][0],
            state[col][1],
            state[col][2],
            state[col][3]
        ];

        novo_state[col][0] = gf_mul14(col_atual[0]) ^ gf_mul11(col_atual[1]) ^ gf_mul13(col_atual[2]) ^ gf_mul9(col_atual[3]);
        novo_state[col][1] = gf_mul9(col_atual[0]) ^ gf_mul14(col_atual[1]) ^ gf_mul11(col_atual[2]) ^ gf_mul13(col_atual[3]);
        novo_state[col][2] = gf_mul13(col_atual[0]) ^ gf_mul9(col_atual[1]) ^ gf_mul14(col_atual[2]) ^ gf_mul11(col_atual[3]);
        novo_state[col][3] = gf_mul11(col_atual[0]) ^ gf_mul13(col_atual[1]) ^ gf_mul9(col_atual[2]) ^ gf_mul14(col_atual[3]);
    }

    novo_state
}

fn pkcs7_pad(plaintext: &[u8]) -> Vec<u8>{
    let block_size = 16;
    let pad_len = block_size - (plaintext.len() % block_size);
    let mut padded = plaintext.to_vec();
    padded.extend(vec![pad_len as u8; pad_len]);
    padded
}

fn pkcs7_unpad(padded: &[u8]) -> Vec<u8>{
    if padded.is_empty(){
        return Vec::new();
    }

    let pad_len = *padded.last().unwrap() as usize;

    if pad_len == 0 || pad_len > 16 || pad_len > padded.len(){
        eprintln!("\nErro: Senha incorreta ou arquivo corrompido.");
        std::process::exit(1);
    }

    padded[..padded.len() - pad_len].to_vec()
}

pub fn aes_encrypt_cbc(plaintext: &[u8], dk: &[u8], dklen: u32) -> Vec<u8>{
    let plaintext = pkcs7_pad(plaintext);
    let subchaves = key_expansion(dk, dklen);
    let num_rounds = (dklen / 4) + 6;

    let mut ciphertext: Vec<u8> = Vec::new();
    let mut iv = [0u8; 16];
    OsRng.fill_bytes(&mut iv);

    // CBC: bloco ^ IV -> AES -> ciphertext
    let mut bloco_anterior = iv;

    for bloco in plaintext.chunks(16) {
        let bloco_atual: [u8; 16] = bloco.try_into().unwrap();

        let mut bloco_resultado = [0u8; 16];
        for i in 0..16{
            bloco_resultado[i] = bloco_atual[i] ^ bloco_anterior[i];
        }

        let mut state = bytes_to_state(bloco_resultado);

        state = add_round_key(state, &subchaves[0]);

        for round in 1..num_rounds{
            state = sub_bytes(state);
            state = shift_rows(state);
            state = mix_columns(state);
            state = add_round_key(state, &subchaves[round as usize]);
        }

        state = sub_bytes(state);
        state = shift_rows(state);
        state = add_round_key(state, &subchaves[num_rounds as usize]);

        let mut bloco_cifrado = [0u8; 16];
        
        let mut i = 0;
        for col in 0..4 {
            for lin in 0..4 {
                bloco_cifrado[i] = state[col][lin];
                i += 1;
            }
        }
        
        ciphertext.extend_from_slice(&bloco_cifrado);

        bloco_anterior = bloco_cifrado;
    }


    let mut resultado_final = Vec::new();
    resultado_final.extend_from_slice(&iv);
    resultado_final.extend_from_slice(&ciphertext);

    resultado_final
}

pub fn aes_decrypt_cbc(iv_ciphertext: &[u8], dk: &[u8], dklen: u32) -> Vec<u8>{
    let mut plaintext: Vec<u8> = Vec::new();

    let subchaves = key_expansion(dk, dklen);
    let num_rounds = (dklen / 4) + 6;

    let iv: [u8; 16] = iv_ciphertext[..16].try_into().unwrap();
    let ciphertext = &iv_ciphertext[16..];

    let mut bloco_anterior = iv;
    
    for bloco in ciphertext.chunks(16){
        let bloco_atual: [u8; 16] = bloco.try_into().unwrap();

        let mut state = bytes_to_state(bloco_atual);
        state = add_round_key(state, &subchaves[num_rounds as usize]);
        state = inv_shift_rows(state);
        state = inv_sub_bytes(state);

        for round in (1..num_rounds).rev(){
            state = add_round_key(state, &subchaves[round as usize]);
            state = inv_mix_columns(state);
            state = inv_shift_rows(state);
            state = inv_sub_bytes(state);
        }
        
        state = add_round_key(state, &subchaves[0]);

        let mut bloco_decifrado = [0u8; 16];
        
        let mut i = 0;
        for col in 0..4 {
            for lin in 0..4 {
                bloco_decifrado[i] = state[col][lin];
                i += 1;
            }
        }

        let mut bloco_resultado = [0u8; 16];
        for i in 0..16{
            bloco_resultado[i] = bloco_decifrado[i] ^ bloco_anterior[i];
        }

        plaintext.extend_from_slice(&bloco_resultado);

        bloco_anterior = bloco_atual;
    }

    pkcs7_unpad(&plaintext)
}

pub fn aes_encrypt_cbc_teste_nist(plaintext: &[u8], dk: &[u8], dklen: u32, iv: [u8; 16]) -> Vec<u8> {
    let subchaves = key_expansion(dk, dklen);
    let num_rounds = (dklen / 4) + 6;
    let mut ciphertext: Vec<u8> = Vec::new();
    let mut bloco_anterior = iv;

    for bloco in plaintext.chunks(16) {
        let bloco_atual: [u8; 16] = bloco.try_into().unwrap();

        let mut bloco_resultado = [0u8; 16];
        for i in 0..16 {
            bloco_resultado[i] = bloco_atual[i] ^ bloco_anterior[i];
        }

        let mut state = bytes_to_state(bloco_resultado);
        state = add_round_key(state, &subchaves[0]);

        for round in 1..num_rounds {
            state = sub_bytes(state);
            state = shift_rows(state);
            state = mix_columns(state);
            state = add_round_key(state, &subchaves[round as usize]);
        }

        state = sub_bytes(state);
        state = shift_rows(state);
        state = add_round_key(state, &subchaves[num_rounds as usize]);

        let mut bloco_cifrado = [0u8; 16];
        let mut i = 0;
        for col in 0..4 {
            for lin in 0..4 {
                bloco_cifrado[i] = state[col][lin];
                i += 1;
            }
        }

        ciphertext.extend_from_slice(&bloco_cifrado);
        bloco_anterior = bloco_cifrado;
    }

    ciphertext
}