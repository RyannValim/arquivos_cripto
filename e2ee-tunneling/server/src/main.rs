// ============================================================
// e2ee-chat-server — servidor cego (blind relay)
//
// O servidor NUNCA vê conteúdo em plaintext.
// Ele só:
//   1. Registra chaves públicas RSA dos clientes
//   2. Roteia mensagens cifradas entre clientes
//   3. Distribui atualizações de presença
// ============================================================

use std::{net::SocketAddr, sync::Arc};

use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

// ------------------------------------------------------------------
// Tipos de mensagem (protocolo cliente <-> servidor)
// ------------------------------------------------------------------

/// Mensagens que o CLIENTE envia para o servidor
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMsg {
    /// Passo 1 do handshake: cliente registra seu nome e chave pública RSA
    Register {
        username: String,
        public_key: PublicKeyDto,
    },

    /// Passo 2: cliente A pede a chave pública de cliente B
    GetKey { target_username: String },

    /// Passo 3+: cliente envia mensagem cifrada para outro cliente
    /// O payload já chegou cifrado — o servidor não toca
    SendMessage {
        to: String,
        /// Chave de sessão AES cifrada com RSA da chave pública do destinatário
        encrypted_session_key: String, // hex
        /// IV + ciphertext AES-CBC, hex-encoded
        encrypted_payload: String, // hex: IV(16) + ciphertext
        /// HMAC-SHA256 do encrypted_payload, hex-encoded
        mac: String, // hex
    },
}

/// Mensagens que o SERVIDOR envia para o cliente
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMsg {
    /// Confirmação de registro com o id gerado
    Registered { user_id: String },

    /// Resposta ao GetKey
    KeyResponse {
        username: String,
        public_key: PublicKeyDto,
    },

    /// Entrega de mensagem cifrada ao destinatário
    IncomingMessage {
        from: String,
        encrypted_session_key: String,
        encrypted_payload: String,
        mac: String,
    },

    /// Lista de usuários online (enviada ao conectar e quando muda)
    UserList { users: Vec<String> },

    /// Erro genérico
    Error { message: String },
}

/// Representação serializável de uma chave pública RSA
/// Usa os campos do seu PublicKey { n: u64, e: u64 }
/// n e e chegam como strings do JS (BigInt não serializa como número JSON)
#[derive(Debug, Serialize, Deserialize, Clone)]
struct PublicKeyDto {
    #[serde(deserialize_with = "deser_u64_or_string")]
    n: u64,
    #[serde(deserialize_with = "deser_u64_or_string")]
    e: u64,
}

/// Aceita tanto número quanto string para u64
/// Necessário porque JS BigInt serializa como string: {"n":"67591"}
fn deser_u64_or_string<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    use serde::de::{self, Visitor};
    struct U64OrString;
    impl<'de> Visitor<'de> for U64OrString {
        type Value = u64;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "u64 ou string numérica")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<u64, E> { Ok(v) }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<u64, E> {
            v.parse().map_err(de::Error::custom)
        }
    }
    d.deserialize_any(U64OrString)
}

// ------------------------------------------------------------------
// Estado compartilhado entre todas as conexões
// ------------------------------------------------------------------

struct UserEntry {
    username: String,
    public_key: PublicKeyDto,
    /// Canal para enviar mensagens a este cliente específico
    sender: broadcast::Sender<String>,
}

type State = Arc<DashMap<String, UserEntry>>; // key = username

// ------------------------------------------------------------------
// Entry point
// ------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await.expect("Falha ao bind");
    println!("[servidor] escutando em ws://{}", addr);

    let state: State = Arc::new(DashMap::new());

    while let Ok((stream, peer)) = listener.accept().await {
        let state = Arc::clone(&state);
        tokio::spawn(handle_connection(stream, peer, state));
    }
}

// ------------------------------------------------------------------
// Handler de cada conexão WebSocket
// ------------------------------------------------------------------

async fn handle_connection(stream: TcpStream, peer: SocketAddr, state: State) {
    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("[{}] handshake WS falhou: {}", peer, e);
            return;
        }
    };

    println!("[{}] conectado", peer);

    let (mut ws_tx, mut ws_rx) = ws.split();

    // Canal broadcast individual para este cliente
    // (capacity 32: até 32 mensagens enfileiradas antes de bloquear)
    let (personal_tx, mut personal_rx) = broadcast::channel::<String>(32);

    // username desta conexão — definido após Register
    let mut my_username: Option<String> = None;

    // Task separada: fica escutando o canal pessoal e despachando ao WS
    let ws_tx_task = tokio::spawn(async move {
        while let Ok(msg) = personal_rx.recv().await {
            if ws_tx.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Loop principal: processa mensagens recebidas do cliente
    while let Some(Ok(msg)) = ws_rx.next().await {
        let text = match msg {
            Message::Text(t) => t.to_string(),
            Message::Close(_) => break,
            _ => continue,
        };

        println!("[{}] recebido: {}", peer, &text);

        let client_msg: ClientMsg = match serde_json::from_str(&text) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("[{}] PARSE ERRO: {} | raw: {}", peer, e, &text);
                send_error(&personal_tx, &format!("JSON inválido: {}", e));
                continue;
            }
        };

        match client_msg {
            // --------------------------------------------------
            // REGISTER: cliente anuncia nome + chave pública
            // --------------------------------------------------
            ClientMsg::Register { username, public_key } => {
                if state.contains_key(&username) {
                    send_error(&personal_tx, "Username já em uso.");
                    continue;
                }

                let user_id = Uuid::new_v4().to_string();

                state.insert(
                    username.clone(),
                    UserEntry {
                        username: username.clone(),
                        public_key,
                        sender: personal_tx.clone(),
                    },
                );

                my_username = Some(username.clone());

                // Confirma registro
                send_msg(
                    &personal_tx,
                    &ServerMsg::Registered {
                        user_id: user_id.clone(),
                    },
                );

                // Envia lista de usuários online para todos
                broadcast_user_list(&state);

                println!("[{}] registrado como '{}'", peer, username);
            }

            // --------------------------------------------------
            // GET_KEY: cliente A quer a chave pública de B
            // --------------------------------------------------
            ClientMsg::GetKey { target_username } => {
                match state.get(&target_username) {
                    Some(entry) => {
                        send_msg(
                            &personal_tx,
                            &ServerMsg::KeyResponse {
                                username: entry.username.clone(),
                                public_key: entry.public_key.clone(),
                            },
                        );
                    }
                    None => {
                        send_error(
                            &personal_tx,
                            &format!("Usuário '{}' não encontrado.", target_username),
                        );
                    }
                }
            }

            // --------------------------------------------------
            // SEND_MESSAGE: roteamento cego
            // O servidor NÃO decifra nada — apenas encaminha
            // --------------------------------------------------
            ClientMsg::SendMessage {
                to,
                encrypted_session_key,
                encrypted_payload,
                mac,
            } => {
                let from = match &my_username {
                    Some(u) => u.clone(),
                    None => {
                        send_error(&personal_tx, "Você precisa se registrar primeiro.");
                        continue;
                    }
                };

                match state.get(&to) {
                    Some(dest) => {
                        send_msg(
                            &dest.sender,
                            &ServerMsg::IncomingMessage {
                                from,
                                encrypted_session_key,
                                encrypted_payload,
                                mac,
                            },
                        );
                    }
                    None => {
                        send_error(
                            &personal_tx,
                            &format!("Destinatário '{}' não está online.", to),
                        );
                    }
                }
            }
        }
    }

    // Cleanup ao desconectar
    if let Some(username) = my_username {
        state.remove(&username);
        broadcast_user_list(&state);
        println!("[{}] '{}' desconectou", peer, username);
    }

    ws_tx_task.abort();
}

// ------------------------------------------------------------------
// Helpers
// ------------------------------------------------------------------

fn send_msg(tx: &broadcast::Sender<String>, msg: &ServerMsg) {
    if let Ok(json) = serde_json::to_string(msg) {
        let _ = tx.send(json);
    }
}

fn send_error(tx: &broadcast::Sender<String>, message: &str) {
    send_msg(tx, &ServerMsg::Error { message: message.to_string() });
}

fn broadcast_user_list(state: &State) {
    let users: Vec<String> = state.iter().map(|e| e.username.clone()).collect();
    let msg = ServerMsg::UserList { users };
    let json = serde_json::to_string(&msg).unwrap();

    for entry in state.iter() {
        let _ = entry.sender.send(json.clone());
    }
}