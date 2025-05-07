import * as THREE from 'three';
import * as CANNON from 'cannon-es';

import { GLTFLoader, RGBELoader } from 'three/examples/jsm/Addons.js';

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
let oppositionTableTarget;
let cursorPosition;
let playerBat;
let playerBatBody;
let lerpSpeed = 0.2;

//---- Cannon physics world setup
const world = new CANNON.World();
world.gravity.set(0, -9.82, 0); // gravity in y-axis

//---- Bat and ball setup in Three.js and Cannon
const batGeometry = new THREE.BoxGeometry(7, 0.1, 16);
const batMaterial = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const bat = new THREE.Mesh(batGeometry, batMaterial);
bat.position.set(0, 1, 0);
scene.add(bat);

playerTableTarget= new THREE.Vector3(0,.05,-8);
oppositionTableTarget= new THREE.Vector3(0,.05,8);

//---- Create a physics body for the bat
const tableBody = new CANNON.Body({
  mass: 0,
  position: new CANNON.Vec3(0, 1, 0),
});
tableBody.addShape(new CANNON.Box(new CANNON.Vec3(0.5, 0.05, 0.15)));
world.addBody(tableBody);

// ----Loading Bat



rgbeLoader.load('src/brown_photostudio_02_2k.hdr', function (texture) {
  const pmremGenerator = new THREE.PMREMGenerator(renderer);
  const envMap = pmremGenerator.fromEquirectangular(texture).texture;

  texture.mapping = THREE.EquirectangularReflectionMapping;
  scene.environment = envMap;

  //render();
 
  // texture.dispose();
  // pmremGenerator.dispose();


  gltfloader.load('src/bat.gltf', function (gltf) {
      console.log("GLTF Model Loaded");
  
      let model = gltf.scene;
      
      model.scale.set(.5, .5, .5); // Adjust scale as needed

      model.castShadow = true;
      model.receiveShadow = true;

      playerBat = model;

      scene.add(model);

      const box = new THREE.Box3().setFromObject(model);
      const size = new THREE.Vector3();
      box.getSize(size);
    
      // Half-extents for Cannon.js box
      const halfExtents = new CANNON.Vec3(size.x / 2, size.y / 2, size.z / 2);
    
      // Create Cannon.js body
      playerBatBody = new CANNON.Body({
        mass: 0, // static
        position: new CANNON.Vec3(model.position.x, model.position.y, model.position.z),
        shape: new CANNON.Box(halfExtents)
      });
    
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
  mass: 0.2,
  position: new CANNON.Vec3(0, 3, 0),
});
ballBody.addShape(new CANNON.Sphere(0.2));
world.addBody(ballBody);

//---- Set camera position
camera.position.z = 13;
camera.position.y = 3;

// ----Detect collision and trigger animation
world.addEventListener("postStep", () => {
  const dist = ballBody.position.distanceTo(playerBatBody.position);
  if (dist < 0.3) {
    triggerBallAnimation();
    console.log("hit player bat");
  }
});

document.addEventListener("mouseup",triggerBallAnimation);

let isBallInAir = false;
let targetPos = new THREE.Vector3(0, 0, 18); // Target position
let startPos = ballBody.position.clone();
let flightDuration = 2; // Duration for the ball's path (seconds)
let startTime = 0;

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
    const curvePos = new THREE.Vector3().lerpVectors(startPos, targetPos, t);
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


let targetZ=9; //TargetZ used to decide the z value the bat is placed on.
let rotationTargetPlayer = new THREE.Vector3(0,0,targetZ);
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
  playerBatBody.position.copy(currentPosition);

//Rotating towards base.
   playerBat.lookAt(rotationTargetPlayer);
   playerBat.rotateX(-Math.PI/2);
   playerBat.rotateY(Math.PI/2);

   //Rotate collision mesh
   playerBatBody.position.copy(currentPosition);
  playerBatBody.quaternion.copy(playerBat);



}


//---- Animation loop
function animate() {
  requestAnimationFrame(animate);

  // Update physics world
  world.step(1 / 60);

  // Sync Three.js objects with Cannon physics bodies
  ball.position.copy(ballBody.position);


  bat.position.copy(tableBody.position);

  // bat.position.x = mouse.x;
  // bat.position.y = mouse.y;
 // bat.rotation.setFromRotationMatrix(batBody.quaternion);

 //playerBat.position.x = mouse.x;
 //playerBat.position.y = mouse.y;

 updatePlayerPosition();

  renderer.render(scene, camera);
}

animate();
