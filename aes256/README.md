# aes256

Implementação do algoritmo AES-256-CBC do zero em Rust, sem bibliotecas externas para a lógica de cifração.

## O que faz

Cifra e decifra arquivos usando AES-256-CBC com chave derivada via PBKDF2-HMAC-SHA256 a partir de uma senha.

## Como rodar

```bash
cargo run -- cifrar <arquivo>
cargo run -- decifrar <arquivo.cifrado>
```

**Exemplo:**

```bash
cargo run -- cifrar documento.txt
# Digite a senha para cifrar: ••••
# Arquivo cifrado salvo em: documento.txt.cifrado

cargo run -- decifrar documento.txt.cifrado
# Digite a senha para decifrar: ••••
# <conteúdo do arquivo>
```

## Formato do arquivo cifrado

```
[ salt (16 bytes) ] [ IV (16 bytes) ] [ ciphertext ]
```

* O salt é gerado aleatoriamente a cada cifragem via `OsRng`.
* O IV é gerado aleatoriamente e embutido no ciphertext.
* A chave AES é derivada da senha com PBKDF2-HMAC-SHA256 (100.000 iterações).

## Comandos disponíveis

| Comando                        | Descrição                      |
| ------------------------------ | -------------------------------- |
| `cifrar <arquivo>`           | Cifra um arquivo com AES-256-CBC |
| `decifrar <arquivo>`         | Decifra um arquivo `.cifrado`  |
| `hash <arquivo>`             | Gera o hash SHA-256 do arquivo   |
| `verificar <arquivo> <hash>` | Verifica autenticidade pelo hash |
| `testar`                     | Roda o vetor de teste do NIST    |

## Dependências

```toml
rand       = "0.8"
rpassword  = "7"
hmac       = "0.12"
sha2       = "0.10"
hex        = "0.4"
```
