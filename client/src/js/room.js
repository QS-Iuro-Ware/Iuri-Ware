function registerRoom() {
  registerEvent("room", "#send", "click", createMessage);
  registerEvent("room", "#text", "keyup", sendOnEnter);

  monitorQueue("room", "messages", showMessage);
  monitorQueue("room", "startGame", startGame);
  monitorQueue("room", "endGame", stopGame);
  monitorQueue("room", "gameInput", gameInput);
}

function createMessage(ev) {
  sendMessage(extractValue("#text"));
  document.querySelector("#text").focus();
}

function sendOnEnter(ev) {
  if (ev.keyCode === 13) {
    createMessage(ev);
    ev.preventDefault();
  }
}

function gameInput({ game, input }) {
  switch (game) {
    case "RockPapiuroScissor":
      sendRockPapiuroScissor(input);
      break;
    case "TheRightIuro":
      sendTheRightIuro(input);
      break;  
  }
}

function clearDiv(id){
  const div = document.getElementById(id);
  while (div.firstChild) {
    div.firstChild.remove();
  }
}

function addIuroImage(target_id, offset){
  var sources = ["img/triangle-Sheet.png", "img/square-Sheet.png", "img/circle-Sheet.png"];
  var file = sources[Math.floor((offset%36)/12)];
  var image = document.createElement("div");
  image.setAttribute("id", "img");
  image.style.background = "url("+file+")";
  image.style.backgroundPositionX = -128*(offset%12)+"px";
  document.querySelector("#"+target_id).appendChild(image);
}

function sendRockPapiuroScissor(button) {
  showMessage(titleCase(name || "You") + " threw " + button.toLowerCase());
  sendRockPapiuroScissorInput(button);
}

function sendTheRightIuro(button) {
  clearDiv("TheRightIuro");
  var div = document.querySelector("#TheRightIuro");
  div.style.gridTemplateColumns = "";
  var img = document.createElement("img");
  img.src = "img/clock.jpeg";
  div.appendChild(img);
  sendTheRightIuroInput(button);
}

function startGame(game) {
  switch (game) {
    case "RockPapiuroScissor":
      startRockPapiuroScissor();
      break;
    default:
      startTheRightIuro(game);
      break;
  }
}

function startRockPapiuroScissor() {
  showMessage("RockPapiuroScissor starting, play your hand");
  document.querySelector("#RockPapiuroScissor").style = "";
}

function startTheRightIuro(game) {
  var offsets = game.TheRightIuro;
  
  showMessage("The Right Iuro is starting, you have 2 seconds to memorize this dude");
  // show right image for 2 seconds
  addIuroImage("TheRightIuro", offsets[0]);
  document.querySelector("#TheRightIuro").style = "";
  
  setTimeout(function(){
     
    clearDiv("TheRightIuro");
    document.querySelector("#TheRightIuro").style.gridTemplateColumns = "minmax(150px, 200px) minmax(150px, 200px) minmax(150px, 200px)";
    
    offsets.sort(() => .5 - Math.random()).forEach((value, index) => {
      
      var img_wrapper = document.createElement("div");
      img_wrapper.setAttribute("id", "img_wrapper"+index);
      document.querySelector("#TheRightIuro").appendChild(img_wrapper);
      // add random images and selection buttons
      var radioInput = document.createElement('input');
      if(index == 0)
        radioInput.checked = true;
      radioInput.setAttribute("type", "radio");
      radioInput.setAttribute("name", "iuro_selection");
      radioInput.setAttribute("value", value);
      img_wrapper.appendChild(radioInput);
      addIuroImage("img_wrapper"+index, value);
    });
    var button = document.createElement("input");
    button.setAttribute("type", "button");
    button.setAttribute("value", "Enviar");
    button.setAttribute("onClick", "data.gameInput.push({ game: 'TheRightIuro', input: [parseInt(document.querySelector('input[name=\"iuro_selection\"]:checked').value)] })");
    document.querySelector("#TheRightIuro").appendChild(button);
  }, 2000);
}

function stopGame(game) {
  switch (game) {
    case "RockPapiuroScissor":
      stopRockPapiuroScissor();
      break;
    case "TheRightIuro":
      stopTheRightIuro();
      break;
  }
}

function stopRockPapiuroScissor() {
  showMessage("RockPapiuroScissor ended");
  document.querySelector("#RockPapiuroScissor").style = "display: none;";
}

function stopTheRightIuro() {
  showMessage("TheRightIuro ended");
    clearDiv("TheRightIuro");
    document.querySelector("#RockPapiuroScissor").style = "display: none;";
}

function showMessage(msg) {
  const control = document.querySelector("#chat");
  control.appendChild(document.createTextNode(msg));
  control.innerHTML = control.innerHTML + "<br/>";
  control.scrollTop = control.scrollTop + 1000;
}

registerRoom();
