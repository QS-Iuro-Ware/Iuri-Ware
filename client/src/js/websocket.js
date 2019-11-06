'use strict'

const protocol = (window.location.protocol === 'https:' && 'wss://' || 'ws://');
const uri = protocol + window.location.host + '/ws/';

const ping = new Uint8Array(1);
ping[0] = 0x9;

// Hell yeah global state
let conn = null;
let name = null;
let group = null;

document.addEventListener("DOMContentLoaded", async () => {
  // Ping
  setInterval(() => { try { conn.send(ping) } catch {} }, 5000);

  await connect();

  // Reconnect automatically
  setInterval(async () => { if (conn === null) await connect() }, 3000);
});

async function connect() {
  conn = new WebSocket(uri);
  conn.binaryType = "arraybuffer";
  conn.onmessage = async (e) => await route(parseJson(e.data));
  conn.onclose = () => conn = null;

  if (name !== null) {
    await sendName(name)
  } else {
    await loadPage("name");
    return;
  }

  if (group != null) {
    await sendJoinRoom(group);
  } else {
    await loadPage("join");
  }
}

function parseJson(msg) {
  try {
    return JSON.parse(msg);
  } catch (e) {
    alert("Unable to parse received message: (" + e + ") " + msg);
  }
}
