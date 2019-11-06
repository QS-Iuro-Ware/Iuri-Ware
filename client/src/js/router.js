document.addEventListener("DOMContentLoaded", () => {
  registerPage("name", registerName);
  registerPage("join", registerJoin);
  registerPage("room", registerRoom);
});

function route(obj) {
  console.log(obj);
  if (obj.Rooms != null) {
    const select = document.querySelector("#group");
    select.innerHTML = "";
  
    for (const room of obj.Rooms) {
      const option = document.createElement("option");
      option.value = room;
      option.innerText = room;
      select.appendChild(option);
    }
  } else if (obj.GameStarted != null) {
    startRockPapiuroScissor();
  } else if (obj.GameEnded != null) {
    log("Game ended:");
    log("Points: " + JSON.stringify(obj.GameEnded))
  } else if (obj.Text != null) {
    log(obj.Text);
  } else if (obj.Error != null) {
    alert(obj.Error);
  } else {
    alert("Unknown message:" + e.data);
  }
}
