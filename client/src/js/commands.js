const sendName = (name) => send({ Name: name || extractValue("#name") });
const sendListRooms = () => conn.send('"ListRooms"');
const sendJoinRoom = (group) => send({ Join: group || extractValue("#group") });
const sendCreateRoom = (group) => send({ Join: group || extractValue("#new_group") });
const sendMessage = (msg) => send({ Message: msg || extractValue("#text") });

const sendRockPapiuroScissorInput = (button) => send({ Game: { RockPapiuroScissor: button } });
