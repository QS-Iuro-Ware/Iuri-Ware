function registerName() {
  registerEvent("name", "#set_name", "click", setName);
}

async function setName(ev) {
  name = extractValue("#name");
  sendName(name);
  await loadPage("join");
}

registerName();
