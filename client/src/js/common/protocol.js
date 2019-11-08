'use strict'

const sendListRooms = () => { try { conn.send('"ListRooms"') } catch {} };

const sendName = (name) => send({ Name: name });

const sendJoinRoom = (room) => send({ Join: room });

const sendMessage = (msg) => send({ Message: msg });

const sendRockPapiuroScissorInput = (button) => send({
  Game: { RockPapiuroScissor: button }
});
