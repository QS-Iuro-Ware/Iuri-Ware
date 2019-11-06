function registerJoin() {
  registerEvent("join", "#join_room", "click", joinRoom);
  registerEvent("join", "#create_room", "click", createRoom);
  sendListRooms();
}

function joinRoom(ev) {
  room = extractValue("#group");
  sendJoinRoom(room);
  loadPage("room");
}

function createRoom(ev) {
  sendCreateRoom();
  loadPage("room");
}
