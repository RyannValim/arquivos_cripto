# file_auth

Verificação de autenticidade de arquivos via SHA-256 em Rust, sem bibliotecas externas para a lógica de hash.

## O que faz

Compara o hash SHA-256 de um arquivo com um hash fornecido, confirmando se o arquivo é autêntico ou foi alterado.

## Como rodar

```bash
cargo run -- verificar <arquivo> <hash>
```

**Exemplo:**

```bash
# Primeiro gere o hash com o projeto sha256:
cargo run -- hash documento.txt
# O arquivo: documento.txt
# Gerou o hash: 7fdab197...

# Depois verifique com file_auth:
cargo run -- verificar documento.txt 7fdab197...
# O arquivo é autêntico!

# Se o hash não bater:
cargo run -- verificar documento.txt 000000...
# O arquivo é falso!
```

## Dependências

```toml
hex = "0.4"
```
