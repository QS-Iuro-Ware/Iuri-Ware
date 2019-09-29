'use strict'

const wsUri = (window.location.protocol== 'https:' && 'wss://' || 'ws://') + window.location.host + '/ws/';
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

    document.querySelector("#join").addEventListener("click", (ev) => {
        const group = document.querySelector("#group");
        log('Joining: ' + group.value);
        conn.send(JSON.stringify({ Join: group.value }));
        group.value = "";
        document.querySelector("#name").focus();
        ev.preventDefault();
    });

    document.querySelector("#set_name").addEventListener("click", (ev) => {
        const name_el = document.querySelector("#name");
        name = name_el.value;
        conn.send(JSON.stringify({ Name: name }));
        name_el.value = "";
        document.querySelector("#text").focus();
        ev.preventDefault();
    });

    document.querySelector("#send").addEventListener("click", (ev) => {
        const text = document.querySelector("#text");
        log(name + ': ' + text.value);
        conn.send(JSON.stringify({ Message: text.value }));
        text.value = "";
        text.focus();
        ev.preventDefault();
    });

    document.querySelector("#text").addEventListener("keyup", (ev) => {
        if (e.keyCode === 13) {
            document.querySelector("#send").click();
            ev.preventDefault();
        }
    });
});

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
        if (obj.Rooms != null) {
            const select = document.querySelector("#group");
            select.innerHTML = "";
            for (const room of obj.Rooms) {
                const option = document.createElement("option");
                option.value = room;
                option.innerText = room;
                select.appendChild(option);
            }
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

function disconnect() {
    if (conn != null) {
        log('Disconnecting...');
        conn.close();
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
