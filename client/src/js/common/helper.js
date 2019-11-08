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
