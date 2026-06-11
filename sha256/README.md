# sha256

Implementação do algoritmo SHA-256 do zero em Rust, sem bibliotecas externas para a lógica de hash.

## O que faz

Gera o hash SHA-256 de qualquer arquivo.

## Como rodar

```bash
cargo run -- hash <arquivo>
```

**Exemplo:**

```bash
cargo run -- hash documento.txt
# O arquivo: documento.txt
# Gerou o hash: 7fdab197...
```

## Dependências

```toml
hex = "0.4"
```
