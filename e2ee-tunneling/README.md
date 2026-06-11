# e2ee-tunneling

Chat com criptografia ponta a ponta (E2EE) usando RSA + AES-256-CBC + HMAC-SHA256, implementados do zero. O servidor é  **cego** : nunca vê nenhuma mensagem em texto claro.

## Estrutura

```
e2ee-tunneling/
├── server/          ← Rust (WebSocket relay)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── aes.rs
│       ├── rsa.rs
│       ├── sha256.rs
│       ├── hmac_sha256.rs
│       └── pbkdf2.rs
└── client/          ← Node.js (serve o HTML estático)
    ├── package.json
    ├── server.js
    └── src/
        └── chat.html
```

## Arquitetura de criptografia

```
ENVIO (Alice → Bob)
  1. Gera chave AES-256 de sessão (32 bytes aleatórios)
  2. Cifra a mensagem com AES-256-CBC + IV aleatório
  3. Assina IV + ciphertext com HMAC-SHA256 (Encrypt-then-MAC)
  4. Cifra a chave AES byte a byte com RSA (chave pública de Bob)
  5. Envia { encrypted_session_key, encrypted_payload, mac } → servidor

SERVIDOR
  Recebe bytes opacos → roteia para Bob → nunca decifra nada

RECEBIMENTO (Bob)
  1. Decifra encrypted_session_key com RSA (chave privada de Bob)
  2. Verifica HMAC antes de decifrar (Verify-then-Decrypt)
  3. Decifra payload com AES-256-CBC
```

## Como rodar

### Pré-requisitos

* Rust + Cargo
* Node.js + npm

### 1. Servidor Rust

```bash
cd e2ee-tunneling/server
cargo run
# → escutando em ws://localhost:9001
```

### 2. Cliente Node

```bash
cd e2ee-tunneling/client
npm install
node server.js
# → http://localhost:3000
```

### 3. Abrir dois clientes

Abra em abas separadas ou browsers diferentes:

```
http://localhost:3000/alice?user=Alice
http://localhost:3000/bob?user=Bob
```

1. Cada aba gera um par RSA localmente no browser
2. Clique no outro usuário na barra lateral
3. O handshake acontece automaticamente
4. Digite e envie — o painel lateral mostra o log criptográfico de cada etapa

**A chave privada RSA nunca sai do browser.**

## Dependências

### Rust (server/Cargo.toml)

```toml
tokio             = { version = "1", features = ["full"] }
tokio-tungstenite = "0.24"
tungstenite       = "0.24"
serde             = { version = "1", features = ["derive"] }
serde_json        = "1"
futures-util      = "0.3"
uuid              = { version = "1", features = ["v4"] }
dashmap           = "6"
rand              = "0.8"
hmac              = "0.12"
sha2              = "0.10"
hex               = "0.4"
```

### Node (client/package.json)

```json
"dependencies": {
  "express": "^4.18.2"
}
```
