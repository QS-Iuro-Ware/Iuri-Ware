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
    await (await dynamicFunction("startRockPapiuroScissor"))();
  } else if (obj.GameEnded != null) {
    await (await dynamicFunction("log"))("Game ended:");
    await (await dynamicFunction("log"))("Points: " + JSON.stringify(obj.GameEnded));
    await (await dynamicFunction("endRockPapiuroScissor"))();
  } else if (obj.Text != null) {
    await (await dynamicFunction("log"))(obj.Text);
  } else if (obj.Error != null) {
    alert(obj.Error);
  } else {
    alert("Unknown message:" + e.data);
  }
}
