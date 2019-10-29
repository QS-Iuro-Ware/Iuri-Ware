function registerName() {
  registerEvent("name", "#set_name", "click", setName);
}

function setName(ev) {
  send({ Name: extractValue("#name") });
  loadPage("join");
}
