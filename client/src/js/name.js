function registerName() {
  registerEvent("name", "#set_name", "click", setName);
}

function setName(ev) {
  name = extractValue("#name");
  sendName(name);
  loadPage("join");
}
