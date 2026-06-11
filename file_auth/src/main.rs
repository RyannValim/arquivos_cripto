mod sha256;

use crate::sha256::sha256;

fn verificar(caminho: &str, hash: &str) {
    let message = std::fs::read(caminho).expect("Erro ao ler o arquivo.");
    let hash_caminho: [u8; 32] = sha256(&message);

    if hash == hex::encode(hash_caminho) {
        println!("O arquivo é autêntico!");
    } else {
        println!("O arquivo é falso!");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        println!("Uso: cargo run -- verificar <arquivo> <hash>");
        return;
    }

    match args[1].as_str() {
        "verificar" => verificar(&args[2], &args[3]),
        _ => println!("Comando inválido. Use: verificar <arquivo> <hash>"),
    }
}