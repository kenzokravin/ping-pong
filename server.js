// server.js
// To test changes, must re-run "npm run ws" in cmd.
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
    playerPosition = 0;
    players[id] = { x: 0, y: 0 ,z: 0,pNum:0};

   // socket.send(JSON.stringify({ type: 'init', id }));
    console.log({ type: 'init', id });
  } else {
    playerPosition = 1;
    players[id] = { x: 0, y: 0 ,z:0,pNum:1};
   
   // socket.send(JSON.stringify({ type: 'init', id}));
  }

  socket.send(JSON.stringify({ type: 'init', id,playerPosition}));

  socket.on('message', message => {
    const data = JSON.parse(message);

    if (data.type === 'move') {
      const player = players[id];
      if (player) {
        player.x = data.dx;
        player.y = data.dy;
        player.z = data.dz;
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
