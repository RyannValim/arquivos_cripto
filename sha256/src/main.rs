mod sha256;

use crate::sha256::sha256;

fn calcular_hash(caminho: &str) {
    let message = std::fs::read(caminho).expect("Erro ao ler o arquivo.");
    let hash: [u8; 32] = sha256(&message);
    println!("O arquivo: {}\nGerou o hash: {}", caminho, hex::encode(hash));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Uso: cargo run -- hash <arquivo>");
        return;
    }

    match args[1].as_str() {
        "hash" => calcular_hash(&args[2]),
        _ => println!("Comando inválido. Use: hash <arquivo>"),
    }
}