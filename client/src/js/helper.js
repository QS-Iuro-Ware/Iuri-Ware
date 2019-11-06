const pages = {};

async function loadPage(page) {
  const obj = document.body;
  let text = await (await fetch("html/" + page + ".html")).text();
  text += "<div id='loaded'></div>";

  if (obj.getAttribute("data-name"))
    unregister(obj.getAttribute("data-name"));

  obj.setAttribute("data-name", page);
  obj.innerHTML = "";

  const newDiv = document.createElement("div");
  newDiv.innerHTML = text;
  obj.appendChild(newDiv);

  const interval = setInterval(() => {
    if (document.getElementById("loaded") !== null) {
      pages[page].load();
      clearInterval(interval);
    }
  }, 100);
}

function registerPage(page, func) {
  pages[page] = pages[page] || { load: null, events: [] };
  pages[page].load = func;
}

function registerEvent(page, selector, eventType, func) {
  pages[page] = pages[page] || { load: null, events: [] };
  pages[page].events.push({ selector, eventType, func });
  document.querySelector(selector).addEventListener(eventType, func);
}

function unregister(page) {
  for (const { selector, eventType, func } of pages[page].events) {
      document.querySelector(selector).removeEventListener(eventType, func);
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
  conn.send(JSON.stringify(obj));
}

function titleCase(text) {
  return text[0].toUpperCase() + text.substring(1).toLowerCase();
}
