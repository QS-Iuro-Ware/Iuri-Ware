document.addEventListener("DOMContentLoaded", async () => {
  await registerPage("name", registerName);
  await registerPage("join", registerJoin);
  await registerPage("room", registerRoom);
});

async function route(obj) {
  console.log(obj);

  if (obj.Rooms != null) {
    const select = await querySelector("#group");
    select.innerHTML = "";

    for (const room of obj.Rooms) {
      const option = document.createElement("option");
      option.value = room;
      option.innerText = room;
      select.appendChild(option);
    }
  } else if (obj.GameStarted != null) {
    await startRockPapiuroScissor();
  } else if (obj.GameEnded != null) {
    await log("Game ended:");
    await log("Points: " + JSON.stringify(obj.GameEnded))
  } else if (obj.Text != null) {
    await log(obj.Text);
  } else if (obj.Error != null) {
    alert(obj.Error);
  } else {
    alert("Unknown message:" + e.data);
  }
}
