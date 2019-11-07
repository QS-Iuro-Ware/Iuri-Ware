'use strict'

const sendListRooms = () => conn.send('"ListRooms"');

const sendName = (name) => send({ Name: name });

const sendJoinRoom = (group) => send({ Join: group });

const sendMessage = (msg) => send({ Message: msg });

const sendRockPapiuroScissorInput = (button) => send({
  Game: { RockPapiuroScissor: button }
});
