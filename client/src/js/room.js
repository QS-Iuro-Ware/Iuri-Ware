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

async function sendRockPapiuroScissor(button) {
  await log(titleCase(name || "You") + " threw " + button.toLowerCase());
  await sendRockPapiuroScissorInput(button);
}

async function startRockPapiuroScissor() {
  await log("RockPapiuroScissor starting, play your hand");
  (await querySelector("#RockPapiuroScissor")).style = "";
}

async function log(msg) {
  const control = await querySelector("#log");
  control.appendChild(document.createTextNode(msg));
  control.innerHTML = control.innerHTML + "<br/>";
  control.scrollTop = control.scrollTop + 1000;
}
