'use strict'

const pages = {};
const data = { messages: [], startGame: [], endGame: [], gameInput: [], rooms: [] }

async function loadPage(page) {
  const obj = document.body;
  let text = await (await fetch("html/" + page + ".html")).text();
  // Hack to allow this function to block until html elements are loaded
  text += "<div id='loaded'></div>";

  const dataName = obj.getAttribute("data-name");
  if (dataName !== null) unregisterPage(dataName);

  obj.setAttribute("data-name", page);
  obj.innerHTML = "";

  const newDiv = document.createElement("div");
  newDiv.innerHTML = text;
  obj.appendChild(newDiv);

  // Don't remove this line
  // It ensures this function is blocked until all elements are loaded,
  // So the dynamic js can manipulate them
  await querySelector("#loaded");

  eval(await (await fetch("js/" + page + ".js")).text());
}

function monitorQueue(page, key, func) {
  const interval = setInterval(() => {
    if (data[key].length > 0) func(data[key].shift());
  }, 50);

  pages[page] = pages[page] || [];
  pages[page].push({ interval });
}

function registerEvent(page, selector, eventType, func) {
  pages[page] = pages[page] || [];
  pages[page].push({ selector, eventType, func });
  document.querySelector(selector).addEventListener(eventType, func);
}

function unregisterPage(page) {
  for (const { selector, eventType, func, interval } of pages[page]) {
    if (interval !== undefined) {
      clearInterval(interval);
    } else {
      document.querySelector(selector).removeEventListener(eventType, func);
    }
  }
  delete pages[page];
}

function extractValue(selector) {
  const element = document.querySelector(selector);
  const value = element.value;
  element.value = "";
  return value;
}

function send(obj) {
  console.log(obj);
  const json = JSON.stringify(obj);
  try {
    conn.send(json);
  } catch {}
}

function titleCase(text) {
  return text[0].toUpperCase() + text.substring(1).toLowerCase();
}

function parseJson(msg) {
  try {
    return JSON.parse(msg);
  } catch (e) {
    alert("Unable to parse received message: (" + e + ") " + msg);
  }
}

async function querySelector(selector) {
  const getter = new Promise((resolve, reject) => {
    const interval = setInterval(() => {
      const data = document.querySelector(selector);
      if (data !== null) {
        clearInterval(interval);
        resolve(data);
      }
    }, 50);
  });

  const rejectionMessage = "Element getter timedout";
  const timeout = new Promise((resolve, reject) => {
    setTimeout(() => reject(rejectionMessage), 1500);
  });
  return Promise.race([getter, timeout]);
}

//startGame Functions


function showMessage(msg) {
  const control = document.querySelector("#chat");
  control.appendChild(document.createTextNode(msg));
  control.innerHTML = control.innerHTML + "<br/>";
  control.scrollTop = control.scrollTop + 1000;
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


function startRockPapiuroScissor() {
  showMessage("RockPapiuroScissor starting, play your hand");
  document.querySelector("#RockPapiuroScissor").style = "";
  const options = ['Rock', 'Papiuro', 'Scissor'];
  const sources = ['img/rock.jpg', 'img/paper.png', 'img/scissors.jpg'];

  options.forEach((value, index) => {
    var optionDiv = document.createElement("div");
    var img = document.createElement("img");
    img.setAttribute("id", "img");
    img.src = sources[index];
    optionDiv.appendChild(img);

    var radioInput = document.createElement('input');
    if(index == 0)
      radioInput.checked = true;
    radioInput.setAttribute("type", "radio");
    radioInput.setAttribute("name", "iuro_selection");
    radioInput.setAttribute("value", value);
    optionDiv.appendChild(radioInput);
    document.querySelector("#RockPapiuroScissor").appendChild(optionDiv);
  });

  var button = document.createElement("input");
  button.setAttribute("type", "button");
  button.setAttribute("value", "OK");
  button.setAttribute("onClick", "data.gameInput.push({ game: 'RockPapiuroScissor', input: document.querySelector('input[name=\"iuro_selection\"]:checked').value })");
  document.querySelector("#RockPapiuroScissor").appendChild(button);
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
    button.setAttribute("value", "OK");
    button.setAttribute("onClick", "data.gameInput.push({ game: 'TheRightIuro', input: [parseInt(document.querySelector('input[name=\"iuro_selection\"]:checked').value)] })");
    document.querySelector("#TheRightIuro").appendChild(button);
  }, 2000);
}
