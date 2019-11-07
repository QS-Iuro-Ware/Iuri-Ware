const pages = {};

async function dynamicFunction(func) {
  const extract = () => window[func];
  const check = (data) => typeof(data) === "function";
  const rejectionMessage = "Dynamic function load timedout";
  return buildRetry(extract, check, rejectionMessage);
}

async function querySelector(selector) {
  const extract = () => document.querySelector(selector);
  const check = (data) => data !== null;
  const rejectionMessage = "Element getter timedout";
  return buildRetry(extract, check, rejectionMessage);
}

async function loadPage(page) {
  const obj = document.body;
  let text = await (await fetch("html/" + page + ".html")).text();
  // Hack to allow this function to block until html elements are loaded
  text += "<div id='loaded'></div>";

  const dataName = obj.getAttribute("data-name");
  if (dataName) await unregisterPage(dataName);

  obj.setAttribute("data-name", page);
  obj.innerHTML = "";

  const newDiv = document.createElement("div");
  newDiv.innerHTML = text;
  obj.appendChild(newDiv);

  // Don't remove this line
  // It ensures this function is blocked until all elements are loaded,
  // So they can be queried with `querySelector` defined above
  await querySelector("#loaded");

  eval(await (await fetch("js/" + page + ".js")).text());
}

async function registerEvent(page, selector, eventType, func) {
  pages[page] = pages[page] || [];
  pages[page].push({ selector, eventType, func });
  (await querySelector(selector)).addEventListener(eventType, func);
}

async function unregisterPage(page) {
  for (const { selector, eventType, func } of pages[page]) {
    (await querySelector(selector)).removeEventListener(eventType, func);
  }
  delete pages[page];
}

async function extractValue(selector) {
  const element = await querySelector(selector);
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

function parseJson(msg) {
  try {
    return JSON.parse(msg);
  } catch (e) {
    alert("Unable to parse received message: (" + e + ") " + msg);
  }
}

async function buildRetry(extract, check, rejectionMessage) {
  const loader = new Promise((resolve, reject) => {
    const interval = setInterval(() => {
      const data = extract();
      if (check(data)) {
        clearInterval(interval);
        resolve(data);
      }
    }, 50);
  });

  const timeout = new Promise((resolve, reject) => {
    setTimeout(() => reject(rejectionMessage), 1500);
  });
  return Promise.race([loader, timeout]);
}
