// server.js
const WebSocket = require('ws');
const server = new WebSocket.Server({ port: 8080 });

let players = {};
let playerCount;
let playerPosition;

server.on('connection', socket => {
  const id = Math.random().toString(36).substr(2, 9);
  //players[id] = { x: 0, y: 0 ,playerNum:0};

  playerTotal();

  if(playerCount==0) {

    players[id] = { x: 0, y: 0 ,playerNum:0};

   // socket.send(JSON.stringify({ type: 'init', id }));
    console.log({ type: 'init', id });
  } else {

    players[id] = { x: 0, y: 0 ,playerNum:1};
   
   // socket.send(JSON.stringify({ type: 'init', id}));
  }

  socket.send(JSON.stringify({ type: 'init', id, playerData:players[id] }));

  socket.on('message', message => {
    const data = JSON.parse(message);

    if (data.type === 'move') {
      const player = players[id];
      if (player) {
        player.x += data.dx;
        player.y += data.dy;
      }
    }
  });

  socket.on('close', () => {
    delete players[id];
  });
});

// Broadcast game state 20 times per second
setInterval(() => {
  const snapshot = JSON.stringify({ type: 'state', players });
  server.clients.forEach(client => {
    if (client.readyState === WebSocket.OPEN) {
      client.send(snapshot);
    }
  });
}, 50);

function playerTotal() {
  playerCount = Object.keys(players).length;
}
