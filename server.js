//import * as CANNON from 'cannon-es';

// server.js
// To test changes, must re-run "npm run ws" in cmd.
import WebSocket, { WebSocketServer } from 'ws';
import * as CANNON from 'cannon-es';

const server = new WebSocketServer({ port: 8080 });


const MAX_PLAYERS = 2;
let players = {};
const serializedPlayers = {};
let playerBodies = [];
let playerCount = 0;
let playerPosition;



//May have to decide ball movement from server instead of client.
let ball={x:0,y:4,z:0};
let isBallInAir = false;
let startTime;

//---- Cannon physics world setup
const world = new CANNON.World();
world.gravity.set(0, -9.82, 0); // gravity in y-axis

//---- Create a physics body for the ball
const ballBody = new CANNON.Body({
  mass: 0.05,
  position: new CANNON.Vec3(0, 6, 0),
});
ballBody.addShape(new CANNON.Sphere(0.2));
world.addBody(ballBody);




//Connection runs once per player that connects to server.
server.on('connection', socket => {
  const id = Math.random().toString(36).substr(2, 9);
  players[id] = { x: 0, y: 0 ,z:0};

  playerTotal();
  
  //Checking that only two players are joining.
  if(playerCount > MAX_PLAYERS) {
    socket.send(JSON.stringify({ type: 'error', message: 'Game is full' }));
    socket.close();
  }
  
  let pNumber = Object.keys(players).length;  //This is the index of their join order.

  if(playerCount==0) {
    playerPosition = pNumber;
    players[id] = { x: 0, y: 0 ,z: 0,pNum: pNumber};

  } else {
    playerPosition = pNumber;
    players[id] = { x: 0, y: 0 ,z:0,pNum:pNumber};
   
  }

    //This could be where we init the player bats.
  const paddleBody = createPaddleBodyForIndex(pNumber);
   //players[id] = { paddleBody };

   playerBodies[playerPosition] = paddleBody;
  
  world.addBody(paddleBody);

  
  // for (const [id, paddleBody] of Object.entries(players)) {
  // serializedPlayers[id] = {
  //   x: paddleBody.position.x,
  //   y: paddleBody.position.y,
  //   z: paddleBody.position.z,
  // };
  // }

  socket.send(JSON.stringify({ type: 'init', id,playerPosition}));

  //message runs everytime a message is sent from client.
  socket.on('message', message => {
    const data = JSON.parse(message);

    if (data.type === 'move') {
      const player = players[id];
      if (player) {
        //Setting the x,y,z value of the player with the client movement.
        player.x = data.dx;
        player.y = data.dy;
        player.z = data.dz;

        //Setting the corresponding playerBody (Cannon Collision) with the player x,y,z values.
        playerBodies.forEach(element => {

          if(element.index == player.pNum) {
            element.position.x = player.x;
            element.position.y = player.y;
            element.position.z = player.z;

            //console.log("Moving PhysBody of:" + player.pNum);
          }
          
        });



      }
    }
  });

  socket.on('close', () => {
    delete players[id];
  });
});


// Broadcast game state 
//Decrease right now: 30 fps. Decreasing increases responsiveness but targets high-end systems.
setInterval(() => {

  //This is where we calculate the ball trajectory.
  //This could be done, by sending table size and width via the server
  //From here, we determine the x,y,z coords of the table.
  //Every time the player "hits" we receive msg, from here, calculate the next "bounce","spin" and "end point".
  //Then we create a curve path to calculate across the interval.






  const snapshot = JSON.stringify({ type:'state',players,ball});

  server.clients.forEach(client => {
    if (client.readyState === WebSocket.OPEN) {
      client.send(snapshot);
    }
  });
}, 33);





//--------------------------------------
//---------- FUNCTIONS -----------------
//--------------------------------------




function serializePlayers() {

  //Don't think this will be of use, the playerbody position does not matter, it is only the x,y,z that we already have available that controls it's position.
  //This is because there is no point sending over the coords of the cannon body as they will be set to the user control anyway.

  for (const [id, paddleBody] of Object.entries(players)) {
    serializedPlayers[id] = {
      x: paddleBody.position.x,
      y: paddleBody.position.y,
      z: paddleBody.position.z,


    };
    }

}

function playerTotal() {
  playerCount = Object.keys(players).length;

}

function createPaddleBodyForIndex(index) {

  const cannonCylinder = new CANNON.Cylinder(0.75, 0.75, 0.1, 8); //Spawning bat shaped 
  const q = new CANNON.Quaternion();
  q.setFromEuler(Math.PI / 2, 0, 0); // Rotate 90 degrees around X
  
  cannonCylinder.transformAllPoints(new CANNON.Vec3(), q); // Apply the rotation to the shape

  let playerBatBody = new CANNON.Body({
    mass: 0, // static
    position: new CANNON.Vec3(0, 0, 0),
    shape: cannonCylinder
  });


  // const paddleShape = new CANNON.Box(new CANNON.Vec3(1, 0.2, 2));
  // const body = new CANNON.Body({ mass: 0 });
  // body.addShape(paddleShape);

  if (index === 1) playerBatBody.position.set(-5, 0, 0); // Player 1
  if (index === 2) playerBatBody.position.set(5, 0, 0);  // Player 2

  return playerBatBody;
}

let targetPos ; //new THREE.Vector3(3, 0, 18); // Target position
let startPos ;// ballBody.position.clone();
let flightDuration = 2; // Duration for the ball's path (seconds), this will decrease as game goes on to increase pace of play.

function startBallPath() {
  if (isBallInAir) return;
  isBallInAir = true;
  startTime = Date.now();
  //ballBody.velocity.set(0, 0, 0); // Stop any current movement

  //animateBallPath();
}

function execBallPath() {
  const elapsedTime = (Date.now() - startTime) / 1000;
  if (elapsedTime > flightDuration) {
    isBallInAir = false;
    return;
  }

  const t = elapsedTime / flightDuration;
  const curvePos = new THREE.Vector3().lerpVectors(startPos, targetPos, t);
  curvePos.y += Math.sin(t * Math.PI) * 2; // Curve effect

  ball.position.copy(curvePos);
  ballBody.position.copy(curvePos);

  requestAnimationFrame(animateBallPath);
}


ballBody.addEventListener('collide', function (event) {
  const otherBody = event.body;

  for(let i = 0; i < playerBodies.length;i++) {
    if (playerBodies[i] === otherBody) {
      //shotHit();
      //triggerBallAnimation();
    console.log('Ball hit player ' + i );
    }

  }

  // if (otherBody === playerBatBody) {
  //   shotHit();
  //   triggerBallAnimation();
  //   console.log('Ball hit player bat');
  // }
});
