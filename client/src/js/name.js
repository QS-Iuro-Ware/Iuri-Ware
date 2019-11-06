async function registerName() {
  await registerEvent("name", "#set_name", "click", setName);
}

async function setName(ev) {
  name = await extractValue("#name");
  await sendName(name);
  await loadPage("join");
}
