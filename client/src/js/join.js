function registerJoin() {
  registerEvent("join", "#join_room", "click", joinRoom);
  registerEvent("join", "#create_room", "click", createRoom);
  sendListRooms();

  monitorQueue("join", "rooms", appendRoomOption);
}

function appendRoomOption(room) {
  const option = document.createElement("option");
  option.value = room;
  option.innerText = room;
  document.querySelector("#room").appendChild(option);
}

async function joinRoom(ev) {
  room = extractValue("#room");
  sendJoinRoom(room);
  await loadPage("room");
}

async function createRoom(ev) {
  room = extractValue("#new_room");
  sendJoinRoom(room);
  await loadPage("room");
}

registerJoin();
