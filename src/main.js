import * as THREE from 'three';
import * as CANNON from 'cannon-es';
import CannonDebugger from 'cannon-es-debugger';

import { GLTFLoader, RGBELoader } from 'three/examples/jsm/Addons.js';

//Ws setup

//const socket = new WebSocket('ws://localhost:8080'); //Js setup.

const socket = new WebSocket("ws://127.0.0.1:3000/ws");

let id;
let players;
let dx,dy,dz; //Coords for player bat
let bx,by,bz; // Coords for server ball.
let opponentBats={};
let otherBats=[];
let sBallObj={};
let sBallMesh;
let side;



//---- Scene setup
const gltfloader = new GLTFLoader();
const rgbeLoader = new RGBELoader();
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

let ballPositionTarget;
let playerTableTarget;
let randomTarget;
let oppositionTableTarget;
let cursorPosition;
let playerBat;
let playerBatBody;
let cylinder;
let lerpSpeed = 0.3;

let cameraDist = 13; //How far camera is from centre of table.
let tableWidth = 7; //Table width.
let targetZ=9;  //TargetZ used to decide the z value the bat is placed on.
let rotationTargetPlayer = new THREE.Vector3(0,0,targetZ+.25);
let rotationTargetOpposition = new THREE.Vector3(0,0,-(targetZ+.25));

//---- Cannon physics world setup
const world = new CANNON.World();
world.gravity.set(0, 0, 0); // gravity in y-axis

//---- Bat and ball setup in Three.js and Cannon
const tableGeometry = new THREE.BoxGeometry(tableWidth, 0.1, 16);
const batMaterial = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const bat = new THREE.Mesh(tableGeometry, batMaterial);
bat.position.set(0, 1, 0);
//scene.add(bat);


playerTableTarget= new THREE.Vector3(0,.05,-8);
oppositionTableTarget= new THREE.Vector3(0,.05,8);

//---- Create a physics body for the bat
const tableBody = new CANNON.Body({
  mass: 0,
  position: new CANNON.Vec3(0, 1, 0),
});
tableBody.addShape(new CANNON.Box(new CANNON.Vec3(0.5, 0.05, 0.15)));
//world.addBody(tableBody);

// ----Loading Bat



rgbeLoader.load('src/brown_photostudio_02_2k.hdr', function (texture) {
  const pmremGenerator = new THREE.PMREMGenerator(renderer);
  const envMap = pmremGenerator.fromEquirectangular(texture).texture;

  texture.mapping = THREE.EquirectangularReflectionMapping;
  scene.environment = envMap;

  //render();
 
  // texture.dispose();
  // pmremGenerator.dispose();


  gltfloader.load('src/pong-bat_cent.gltf', function (gltf) {
      console.log("GLTF Model Loaded");
  
      let model = gltf.scene;
      
      model.scale.set(.75, .75, .75); // Adjust scale as needed

      model.castShadow = true;
      model.receiveShadow = true;

      playerBat = model;

      scene.add(model);

      const box = new THREE.Box3().setFromObject(model);

      const cylinderGeo = new THREE.CylinderGeometry(.75,.75,.1,8);
      const cylMaterial = new THREE.MeshBasicMaterial({ color: 0x00ffff ,wireframe:true});
      cylinder = new THREE.Mesh(cylinderGeo, cylMaterial);
      scene.add(cylinder);
      cylinder.position.set(0, 1, 0);


      const cannonCylinder = new CANNON.Cylinder(0.75, 0.75, 0.1, 8);
     // const q = new CANNON.Quaternion();
      //q.setFromEuler(Math.PI / 2, 0, 0); // Rotate 90 degrees around X

      //cannonCylinder.transformAllPoints(new CANNON.Vec3(), q); // Apply the rotation to the shape

      const size = new THREE.Vector3();
      box.getSize(size);
    
      // Half-extents for Cannon.js box
      const halfExtents = new CANNON.Vec3(size.x / 2, size.y / 2, size.z / 2);
    
      // Create Cannon.js body
      playerBatBody = new CANNON.Body({
        mass: 0, // static
        position: new CANNON.Vec3(model.position.x, model.position.y, model.position.z),
        shape: cannonCylinder
      });

      playerBatBody.quaternion.setFromEuler(Math.PI / 2, 0, 0);
    
      world.addBody(playerBatBody);


  });


});

//---- Ball setup in Three.js
const ballGeometry = new THREE.SphereGeometry(0.2);
const ballMaterial = new THREE.MeshBasicMaterial({ color: 0xff0000 });
const ball = new THREE.Mesh(ballGeometry, ballMaterial);
ball.position.set(0, 3, 0);
scene.add(ball);

//---- Create a physics body for the ball
const ballBody = new CANNON.Body({
  mass: 0.001,
  position: new CANNON.Vec3(0, 6, 0),
});
ballBody.addShape(new CANNON.Sphere(0.2));
world.addBody(ballBody);
//ballBody.velocity.set(0,-15,0);

//---- Set camera position
camera.position.z = cameraDist;
camera.position.y = 3;

// ---- WS setup.

socket.addEventListener('message', event => {
  const data = JSON.parse(event.data);

  if(data.type === 'ball_state' ) {
    console.log("ball state: " + data);

  }


  if (data.type === 'init') {
    id = data.id;
    //console.log(data.playerPosition); //This finds what order the player joins the server and how to broadcast others.

    if(data.playerPosition == 1) {
      side = 1;
    } else {
      side = 0;
    }

    setSide(side);
    
  } else if (data.type === 'state') {
    players = data.players; //This is receiving info about all players.
    //console.log(players);

    for (const [playerId, pos] of Object.entries(data.players)) {
      if (playerId === id) continue; // Don't update yourself

      for (const existingId of Object.keys(opponentBats)) {
        if (!data.players.hasOwnProperty(existingId)) {
          scene.remove(opponentBats[existingId]);
          delete opponentBats[existingId];
        }
      }

      //Creating oppositionBatModel.
      if (!opponentBats[playerId]) {
        if (!playerBat) return; //checks if player bat has loaded (gltf loader)

        let batClone = playerBat.clone(); // or load a separate model
        //batClone.material = playerBat.material.clone(); // optional: make it distinct
        scene.add(batClone);
        opponentBats[playerId] = batClone;
        console.log(opponentBats);

        batClone.traverse(child => {
          if (child.isMesh) {
            child.material = new THREE.MeshStandardMaterial({ color: 0xff0000 });
          }
        });
      }


       // Update position
       const opponentBat = opponentBats[playerId];
       opponentBat.position.set( pos.x,pos.y,pos.z);
 
       // Optional: make them look at same point
       opponentBat.lookAt(rotationTargetOpposition);
       opponentBat.rotateX(-Math.PI / 2);
       opponentBat.rotateY(Math.PI / 2);
    }

  //Render ball
 // console.log(data.ball);

  if (!ball) {
    return;
  }

  const sBall = data.ball;

  if(typeof sBallMesh === 'undefined') {
    sBallMesh = ball.clone(); //creates clone of ball.
    scene.add(sBallMesh);

  }



  sBallMesh.position.set(sBall.x,sBall.y,sBall.z);
  ballBody.position.x = sBall.x;
  ballBody.position.y = sBall.y;
  ballBody.position.z = sBall.z;

  //console.log("Ball: " + ballBody.position + " Paddle: " + playerBatBody.position);
  

  }
});

//setSide acts to swap the logic so that the other player is positioned on the other side of the table.
function setSide(side) {
  if(side==0) return;
  
  if (side == 1) {

    camera.position.z = -cameraDist;
    
    camera.rotateY(Math.PI);

    targetZ= -targetZ;

    rotationTargetPlayer = new THREE.Vector3(0,0,targetZ-.25);
    rotationTargetOpposition.z = rotationTargetOpposition.z * -1;

  }

}


// ----Detect collision and trigger animation
world.addEventListener("postStep", () => {
  const dist = ballBody.position.distanceTo(playerBatBody.position);
  if (dist < .10) {
    //shotHit();
   // triggerBallAnimation();
    
    //console.log("hit player bat");
  }
});

const cannonDebugger = CannonDebugger(scene, world, {
  color: 0x00ffaa, // optional
});


//document.addEventListener("mousedown",triggerBallAnimationStart);
//document.addEventListener("mouseup",triggerBallAnimation);

let isBallInAir = false;
let targetPos = new THREE.Vector3(3, 0, 18); // Target position
let startPos = ballBody.position.clone();
let flightDuration = 2; // Duration for the ball's path (seconds)
let startTime = 0;

function triggerBallAnimationStart() {
  if (isBallInAir) return;
  isBallInAir = true;
  startTime = Date.now();
  ballBody.velocity.set(0, 0, 0); // Stop any current movement

  function animateBallPath() {
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

  animateBallPath();
}

function triggerBallAnimation() {
  if (isBallInAir) return;
  isBallInAir = true;
  startTime = Date.now();
  ballBody.velocity.set(0, 0, 0); // Stop any current movement

  function animateBallPath() {
    const elapsedTime = (Date.now() - startTime) / 1000;
    if (elapsedTime > flightDuration) {
      isBallInAir = false;
      return;
    }

    const t = elapsedTime / flightDuration;
    const curvePos = new THREE.Vector3().lerpVectors(startPos, randomTarget, t);
    curvePos.y += Math.sin(t * Math.PI) * 2; // Curve effect

    ball.position.copy(curvePos);
    ballBody.position.copy(curvePos);

    requestAnimationFrame(animateBallPath);
  }

  animateBallPath();
}

let mouse = new THREE.Vector2();
let playRegionMultiplier = new THREE.Vector2(3,2);
let raycaster = new THREE.Raycaster();
const quaternion = new THREE.Quaternion();


 //TargetZ used to decide the z value the bat is placed on.
var vec = new THREE.Vector3(); // create once and reuse
var pos = new THREE.Vector3(); // create once and reuse



document.addEventListener('mousemove', onDocMouseMove);

function onDocMouseMove(event) {
  event.preventDefault();
  mouse.x = ((event.clientX / window.innerWidth) * 2 - 1)*4;
  mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

  //----Determining mouse movement along plane:
  vec.set(
    ( event.clientX / window.innerWidth ) * 2 - 1,
    - ( event.clientY / window.innerHeight ) * 2 + 1,
    0.5,
  );
  
  vec.unproject( camera );
      
  vec.sub( camera.position ).normalize();
      
  var distance =(targetZ - camera.position.z) / vec.z;
      
  pos.copy( camera.position ).add( vec.multiplyScalar( distance ) );


}

raycaster.setFromCamera( mouse.clone(), camera );   

var objects = raycaster.intersectObjects(scene.children);


function updateServerBall() {


  socket.send(JSON.stringify({ type: 'move', dx, dy ,dz}));
}



//---Updates player bat position.
// Within this, rotation will be required for a natural looking turn. (as potential polish.)
function updatePlayerPosition() {
  // Current and target positions
  let currentPosition = new THREE.Vector3().copy(playerBat.position);
  let targetPosition = new THREE.Vector3(mouse.x, mouse.y, 0);

   targetPosition = pos;

   

  // Move towards the target
  currentPosition.lerp(targetPosition, lerpSpeed);
  playerBat.position.copy(currentPosition);
  //playerBatBody.position.copy(currentPosition);

//Rotating towards base.
   playerBat.lookAt(rotationTargetPlayer);
    playerBat.rotateX(-Math.PI/2);
    playerBat.rotateY(Math.PI/2);

   //Rotate collision mesh
   //playerBatBody.position.copy(currentPosition);
 // playerBatBody.quaternion.copy(playerBat);

 // let direction = new THREE.Vector3().subVectors(rotationTargetPlayer, currentPosition);
 // direction.normalize();
  //let offset = direction.multiplyScalar(1);
  // let cylinderCopy =  new THREE.Vector3().copy(currentPosition).sub(offset); //This offset is used for when the origin of the bat is at the handle.
  
  let cylinderCopy = currentPosition;

  //cylinder.position.copy(cylinderCopy);
  cylinder.lookAt(rotationTargetPlayer);

  //console.log(rotationTargetPlayer);
   
  cylinder.rotateZ(Math.PI/2);

  playerBatBody.position.x = playerBat.position.x;
  playerBatBody.position.y = playerBat.position.y;
  playerBatBody.position.z = playerBat.position.z;
  //playerBatBody.quaternion.copy(cylinder);

  //Loading positional data from player to send to websocket so other players can render.
  dx=currentPosition.x;
  dy=currentPosition.y;
  dz=currentPosition.z;

  socket.send(JSON.stringify({ type: 'move', dx, dy ,dz}));
   
}


ballBody.addEventListener('collide', function (event) {
  const otherBody = event.body;

  //This listener works by checking the event body against the player body list.
  console.log("hit other body!!");

  // if (otherBody === playerBatBody) {
  //   shotHit();
  //   triggerBallAnimation();
  //   console.log('Ball hit player bat');
  // }
});


function shotHit() {
  //----This is the shot return function. Takes a randomized target on the table (vector3) and then creates a curve towards it.
  // After, add spin depending on player shot (to achieve this, potentially have two checks, a large distance and small. 
  // The large distance can be used to check bat movement direction and from this, apply spin. The small is used for "hitting" the ball.)
  // Spin application just changes the direction of the ball after the bounce. Creating variation in the hits.

  //Deciding random point on table to target first.
  randomTarget = playerTableTarget;
  randomTarget.x = Math.random() * ((tableWidth/2) - (-(tableWidth/2))) + -(tableWidth/2);
  
  console.log(randomTarget);


}


//---- Animation loop
function animate() {
  requestAnimationFrame(animate);

  // Update physics world
  world.step(1 / 60);

  // Sync Three.js objects with Cannon physics bodies
  ball.position.copy(ballBody.position);



  //bat.position.copy(tableBody.position);

  // bat.position.x = mouse.x;
  // bat.position.y = mouse.y;
 // bat.rotation.setFromRotationMatrix(batBody.quaternion);

//  playerBatBody.position.x = 3;
//  playerBatBody.position.y = 3;

 updatePlayerPosition();
 cannonDebugger.update();

  renderer.render(scene, camera);
}

animate();
