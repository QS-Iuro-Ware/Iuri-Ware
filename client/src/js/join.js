async function registerJoin() {
  await registerEvent("join", "#join_room", "click", joinRoom);
  await registerEvent("join", "#create_room", "click", createRoom);
  sendListRooms();
}

async function joinRoom(ev) {
  room = await extractValue("#group");
  await sendJoinRoom(room);
  await loadPage("room");
}

async function createRoom(ev) {
  await sendCreateRoom();
  await loadPage("room");
}
