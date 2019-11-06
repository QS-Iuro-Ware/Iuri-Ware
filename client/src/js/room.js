function registerRoom() {
  registerEvent("room", "#send", "click", createMessage);
  registerEvent("room", "#text", "keyup", sendOnEnter);
}

function createMessage(ev) {
  sendMessage();
  document.queryElement("#text").focus();
}

function sendOnEnter(ev) {
  if (ev.keyCode === 13) {
    document.querySelector("#send").click();
    ev.preventDefault();
  }
}

function sendRockPapiuroScissor(button) {
  log(titleCase(name || "You") + " threw " + button.toLowerCase());
  sendRockPapiuroScissorInput(button);
}

function startRockPapiuroScissor() {
  const game = document.querySelector("#RockPapiuroScissor");
  if (game === null) {
    setTimeout(() => startRockPapiuroScissor(), 100);
    return;
  }

  log("RockPapiuroScissor starting, play your hand");
  game.style = "";
}

function log(msg) {
  const control = document.querySelector("#log");
  if (control === null) {
    setTimeout(() => log(msg), 100);
    return;
  }

  control.appendChild(document.createTextNode(msg)); 
  control.innerHTML = control.innerHTML + "<br/>";
  control.scrollTop = control.scrollTop + 1000;
}
