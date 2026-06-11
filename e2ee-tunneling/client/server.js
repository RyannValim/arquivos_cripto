// server.js — servidor HTTP simples que serve os dois clientes de chat
// Rodando em http://localhost:3000/alice e http://localhost:3000/bob

import express from "express";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const __dirname = dirname(fileURLToPath(import.meta.url));
const app = express();

app.use(express.static(join(__dirname, "src")));

// Rota /alice e /bob servem o mesmo HTML — o nome é passado como query param
app.get("/alice", (req, res) => {
  res.sendFile(join(__dirname, "src", "chat.html"));
});
app.get("/bob", (req, res) => {
  res.sendFile(join(__dirname, "src", "chat.html"));
});

app.listen(3000, () => {
  console.log("Cliente A → http://localhost:3000/alice?user=Alice");
  console.log("Cliente B → http://localhost:3000/bob?user=Bob");
});
