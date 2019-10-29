function registerJoin() {
  registerEvent("join", "#join_room", "click", joinRoom);
  registerEvent("join", "#create_room", "click", createRoom);
  conn.send('"ListRooms"');
}

function joinRoom(ev) {
  send({ Join: extractValue("#group") })
  loadPage("room");
}

function createRoom(ev) {
  send({ Join: extractValue("#new_group") });
  loadPage("room");
}
