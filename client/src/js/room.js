function registerRoom() {
  registerEvent("room", "#send", "click", sendMessage);
  registerEvent("room", "#text", "keyup", sendOnEnter);
}

function sendMessage(ev) {
  send({ Message: extractValue("#text") });
  document.queryElement("#text").focus();
}

function sendOnEnter(ev) {
  if (ev.keyCode === 13) {
      document.querySelector("#send").click();
      ev.preventDefault();
  }
}

function sendRockPapiuroScissor(button) {
  name = name || "You";
  const titleName = name[0].toUpperCase() + name.substring(1).toLowerCase();
  log(name + " threw " + button.toLowerCase());
  send({ Game: { RockPapiuroScissor: button } });
}

function startRockPapiuroScissor() {
  document.querySelector("#RockPapiuroScissor").style = "";
  log("RockPapiuroScissor starting, play your hand");
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
