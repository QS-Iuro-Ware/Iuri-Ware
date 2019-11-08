'use strict'

const protocol = (window.location.protocol === 'https:' && 'wss://' || 'ws://');
const uri = protocol + window.location.host + '/ws/';

const ping = new Uint8Array(1);
ping[0] = 0x9;

// Hell yeah global state
let conn = null;
let name = null;
let room = null;

document.addEventListener("DOMContentLoaded", async () => {
  // Ping
  setInterval(() => { try { conn.send(ping) } catch {} }, 5000);

  await connect();

  // Reconnects automatically
  setInterval(async () => { if (conn === null) await connect() }, 3000);
});

async function connect() {
  conn = new WebSocket(uri);
  conn.binaryType = "arraybuffer";
  conn.onmessage = async (e) => await route(parseJson(e.data));
  conn.onopen = async () => {
    if (name !== null) {
      sendName(name)
    } else {
      await loadPage("name");
      return;
    }

    if (room !== null) {
      sendJoinRoom(room);
      await loadPage("room");
    } else {
      await loadPage("join");
    }
  }
  conn.onclose = () => conn = null;
}
