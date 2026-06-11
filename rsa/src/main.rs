mod rsa;

use crate::rsa::{gerar_chaves, cifrar, decifrar};

fn main() {
    let (pub_key, priv_key) = gerar_chaves(257, 263, 17);

    println!("Chave pública: (n={}, e={})", pub_key.n, pub_key.e);
    println!("Chave privada: (n={}, d={})", priv_key.n, priv_key.d);

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("\nUso:");
        println!("cargo run -- cifrar <mensagem>");
        println!("cargo run -- decifrar <numeros separados por virgula>");
        return;
    }

    match args[1].as_str() {
        "cifrar" => {
            let texto = &args[2];
            let cifrados: Vec<u64> = texto
                .bytes()
                .map(|b| cifrar(b as u64, &pub_key))
                .collect();
            let resultado: Vec<String> = cifrados.iter().map(|n| n.to_string()).collect();
            println!("\nCifrado: {}", resultado.join(","));
        }
        "decifrar" => {
            let decifrado: String = args[2]
                .split(',')
                .map(|n| n.trim().parse::<u64>().expect("Número inválido"))
                .map(|c| decifrar(c, &priv_key) as u8 as char)
                .collect();
            println!("\nDecifrado: {}", decifrado);
        }
        _ => println!("Comando inválido. Use 'cifrar' ou 'decifrar'."),
    }
}