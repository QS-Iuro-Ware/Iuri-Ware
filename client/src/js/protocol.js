const sendListRooms = () => conn.send('"ListRooms"');

const sendName = async (name) => send({
  Name: name || await extractValue("#name")
});

const sendJoinRoom = async (group) => send({
  Join: group || await extractValue("#group")
});

const sendCreateRoom = async (group) => await send({
  Join: group || await extractValue("#new_group")
});

const sendMessage = async (msg) => send({
  Message: msg || await extractValue("#text")
});

const sendRockPapiuroScissorInput = (button) => send({
  Game: { RockPapiuroScissor: button }
});
