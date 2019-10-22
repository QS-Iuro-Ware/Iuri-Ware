'use strict'

const wsUri = (window.location.protocol == 'https:' && 'wss://' || 'ws://') + window.location.host + '/ws/';
const ping = (() => {
    let ping = new Uint8Array(1);
    ping[0] = 0x9;
    return ping;
})();

// Hell yeah global state
let conn = null;
let name = null;

// Ping
setInterval(() => { try { conn.send(ping) } catch {} }, 5000);

document.addEventListener("DOMContentLoaded", () => {
    connect();

    document.querySelector("#connect").addEventListener("click", (ev) => {
        if (conn == null) {
            connect();
            document.querySelector("#group").focus();
        } else {
            disconnect();
        }
        update_ui();
        ev.preventDefault();
    });

    document.querySelector("#join_room").addEventListener("click", (ev) => {
	const group = extractValue("#group");
        log('Joining: ' + group);
	send({ Join: group })
	focusAndPreventDefault("#name", ev);
    });

    document.querySelector("#create_room").addEventListener("click", (ev) => {
	const group = extractValue("#new_group");
        log('Joining: ' + group);
	send({ Join: group })
	focusAndPreventDefault("#name", ev);
    });

    document.querySelector("#set_name").addEventListener("click", (ev) => {
	name = extractValue("#name");
	send({ Name: name });
	focusAndPreventDefault("#text", ev);
    });

    document.querySelector("#send").addEventListener("click", (ev) => {
	const text = extractValue("#text");
	send({ Message: text });
	focusAndPreventDefault("#text", ev);
    });

    document.querySelector("#text").addEventListener("keyup", (ev) => {
        if (ev.keyCode === 13) {
            document.querySelector("#send").click();
            ev.preventDefault();
        }
    });
});

function focusAndPreventDefault(selector, ev) {
    document.querySelector(selector).focus();
    ev.preventDefault();
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

function log(msg) {
    const control = document.querySelector("#log");
    control.appendChild(document.createTextNode(msg)); 
    control.innerHTML = control.innerHTML + "<br/>";
    control.scrollTop = control.scrollTop + 1000;
}

function connect() {
    disconnect();
    conn = new WebSocket(wsUri);
    log('Connecting...');

    conn.onopen = () => {
        conn.binaryType = "arraybuffer";
        log('Connected.');
        update_ui();
        conn.send('"ListRooms"');
    };

    const parseJson = (msg) => {
        try {
            return JSON.parse(msg);
        } catch (e) {
            alert("Unable to parse received message: (" + e + ") " + msg);
        }
    };


    conn.onmessage = (e) => {
        const obj = parseJson(e.data);
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
	    console.log(obj.GameEnded);
        } else if (obj.Text != null) {
            log(obj.Text);
        } else if (obj.Error != null) {
            alert(obj.Error);
        } else {
            alert("Unknown message:" + e.data);
        }
    };

    conn.onclose = () => {
        log('Disconnected.');
        conn = null;
        update_ui();
    };
}

function sendRockPapiuroScissor(button) {
    send({ Game: { RockPapiuroScissor: button } });
}

function startRockPapiuroScissor() {
    document.querySelector("#RockPapiuroScissor").style = "";
}

function disconnect() {
    if (conn != null) {
        log('Disconnecting...');
        try { conn.close() } catch {};
        conn = null;
        update_ui();
    }
}

function update_ui() {
    if (conn == null) {
        document.querySelector("#status").innerText = "disconnected";
        document.querySelector("#connect").innerText = "Connect";
    } else {
        document.querySelector("#status").innerText = "connected";
        document.querySelector("#connect").innerText = "Disconnect";
    }
}
