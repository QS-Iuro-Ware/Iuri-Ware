'use strict'

const protocol = (window.location.protocol === 'https:' && 'wss://' || 'ws://');
const uri = protocol + window.location.host + '/ws/';

const ping = new Uint8Array(1);
ping[0] = 0x9;

// Hell yeah global state
let conn = null;
let name = null;
let group = null;

document.addEventListener("DOMContentLoaded", () => {
  // Ping
  setInterval(() => { try { conn.send(ping) } catch {} }, 5000);

  // Reconnect
  setInterval(() => { if (conn === null) connect() }, 3000);
  connect();
});

function connect() {
  conn = new WebSocket(uri);
  conn.binaryType = "arraybuffer";
  conn.onmessage = (e) => route(parseJson(e.data));
  conn.onclose = () => conn = null;

  if (name !== null) {
    sendName(name)
  } else {
    loadPage("name");
    return;
  }

  if (group != null) {
    sendJoinRoom(group);
  } else {
    loadPage("join");
  }
}

function parseJson(msg) {
  try {
    return JSON.parse(msg);
  } catch (e) {
    alert("Unable to parse received message: (" + e + ") " + msg);
  }
}
