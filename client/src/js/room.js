function registerRoom() {
  registerEvent("room", "#send", "click", createMessage);
  registerEvent("room", "#text", "keyup", sendOnEnter);

  monitorQueue("room", "messages", showMessage);
  monitorQueue("room", "startGame", startGame);
  monitorQueue("room", "endGame", stopGame);
  monitorQueue("room", "gameInput", gameInput);
}

function createMessage(ev) {
  sendMessage(extractValue("#text"));
  document.querySelector("#text").focus();
}

function sendOnEnter(ev) {
  if (ev.keyCode === 13) {
    createMessage(ev);
    ev.preventDefault();
  }
}

function gameInput({ game, input }) {
  switch (game) {
  case "RockPapiuroScissor":
    sendRockPapiuroScissor(input);
    break;
  }
}

function sendRockPapiuroScissor(button) {
  showMessage(titleCase(name || "You") + " threw " + button.toLowerCase());
  sendRockPapiuroScissorInput(button);
}

function startGame(game) {
  switch (game) {
  case "RockPapiuroScissor":
    startRockPapiuroScissor();
    break;
  }
}

function startRockPapiuroScissor() {
  showMessage("RockPapiuroScissor starting, play your hand");
  document.querySelector("#RockPapiuroScissor").style = "";
}

function stopGame(game) {
  switch (game) {
  case "RockPapiuroScissor":
    stopRockPapiuroScissor();
    break;
  }
}

function stopRockPapiuroScissor() {
  showMessage("RockPapiuroScissor ended");
  document.querySelector("#RockPapiuroScissor").style = "display: none;";
}

function showMessage(msg) {
  const control = document.querySelector("#chat");
  control.appendChild(document.createTextNode(msg));
  control.innerHTML = control.innerHTML + "<br/>";
  control.scrollTop = control.scrollTop + 1000;
}

registerRoom();
