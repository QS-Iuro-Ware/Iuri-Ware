async function registerRoom() {
  await registerEvent("room", "#send", "click", createMessage);
  await registerEvent("room", "#text", "keyup", sendOnEnter);
}

async function createMessage(ev) {
  await sendMessage();
  (await querySelector("#text")).focus();
}

async function sendOnEnter(ev) {
  if (ev.keyCode === 13) {
    (await querySelector("#send")).click();
    ev.preventDefault();
  }
}

// If a function must be used from html or `router` you must set it as a `window` attribute
window.sendRockPapiuroScissor = async (button) => {
  await log(titleCase(name || "You") + " threw " + button.toLowerCase());
  await sendRockPapiuroScissorInput(button);
}

window.startRockPapiuroScissor = async () => {
  await log("RockPapiuroScissor starting, play your hand");
  (await querySelector("#RockPapiuroScissor")).style = "";
}

window.stopRockPapiuroScissor = async () => {
  await log("RockPapiuroScissor ended");
  (await querySelector("#RockPapiuroScissor")).style = "display: none;";
}

window.log = async (msg) => {
  const control = await querySelector("#chat");
  control.appendChild(document.createTextNode(msg));
  control.innerHTML = control.innerHTML + "<br/>";
  control.scrollTop = control.scrollTop + 1000;
}

registerRoom();
