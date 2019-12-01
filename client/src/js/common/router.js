'use strict'

// The router can only talk with the document through queues
// Append it to the queue here and monitor the queue in the page's dynamic js
async function route(obj) {
  console.log(obj);

  if (obj.Rooms != null) {
    data.rooms = obj.Rooms;
  } else if (obj.GameStarted != null) {
    // The timeout allows games to end before starting another,
    // so we don't race and close the game that just started
    setTimeout(() => data.startGame.push(obj.GameStarted), 100);
  } else if (obj.GameEnded != null) {
    data.messages.push("Game " + obj.GameEnded[0] + " ended");
    data.endGame.push(obj.GameEnded[0]);
  } else if (obj.Text != null) {
    data.messages.push(obj.Text);
  } else if (obj.Error != null) {
    alert(obj.Error);
  } else {
    alert("Unknown message:" + e.data);
  }
}
