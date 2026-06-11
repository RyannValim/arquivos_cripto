# rsa

Implementação do algoritmo RSA do zero em Rust, sem bibliotecas externas para a criptografia.

## O que faz

Cifra e decifra mensagens de texto usando RSA com chaves geradas a partir de dois números primos.

## Como rodar

```bash
cargo run -- cifrar "<mensagem>"
cargo run -- decifrar "<numeros separados por virgula>"
```

**Exemplo:**

```bash
cargo run -- cifrar "ola"
# Chave pública: (n=67591, e=17)
# Chave privada: (n=67591, d=3953)
# Cifrado: 12345,67890,11111

cargo run -- decifrar "12345,67890,11111"
# Chave pública: (n=67591, e=17)
# Chave privada: (n=67591, d=3953)
# Decifrado: ola
```

## Observações

* Os primos usados são `p=257`, `q=263`, `e=17` — fixos no código, adequados para fins didáticos.
* Cada byte da mensagem é cifrado individualmente com `c = m^e mod n`.
* A mensagem deve ter todos os bytes menores que `n` (67591), o que cobre todos os caracteres ASCII.

## Dependências

Nenhuma dependência externa.
