import * as THREE from 'three';
import * as CANNON from 'cannon-es';

// Scene setup
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Cannon physics world setup
const world = new CANNON.World();
world.gravity.set(0, -9.82, 0); // gravity in y-axis

// Bat and ball setup in Three.js and Cannon
const batGeometry = new THREE.BoxGeometry(1, 0.1, 0.3);
const batMaterial = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const bat = new THREE.Mesh(batGeometry, batMaterial);
bat.position.set(0, 1, 0);
scene.add(bat);

// Create a physics body for the bat
const batBody = new CANNON.Body({
  mass: 1,
  position: new CANNON.Vec3(0, 1, 0),
});
batBody.addShape(new CANNON.Box(new CANNON.Vec3(0.5, 0.05, 0.15)));
world.addBody(batBody);

// Ball setup in Three.js
const ballGeometry = new THREE.SphereGeometry(0.2);
const ballMaterial = new THREE.MeshBasicMaterial({ color: 0xff0000 });
const ball = new THREE.Mesh(ballGeometry, ballMaterial);
ball.position.set(0, 3, 0);
scene.add(ball);

// Create a physics body for the ball
const ballBody = new CANNON.Body({
  mass: 1,
  position: new CANNON.Vec3(0, 3, 0),
});
ballBody.addShape(new CANNON.Sphere(0.2));
world.addBody(ballBody);

// Set camera position
camera.position.z = 5;

// Detect collision and trigger animation
world.addEventListener("postStep", () => {
  const dist = ballBody.position.distanceTo(batBody.position);
  if (dist < 0.3) {
    triggerBallAnimation();
  }
});

let isBallInAir = false;
let targetPos = new THREE.Vector3(5, 5, 0); // Target position
let startPos = ball.position.clone();
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

// Animation loop
function animate() {
  requestAnimationFrame(animate);

  // Update physics world
  world.step(1 / 60);

  // Sync Three.js objects with Cannon physics bodies
  ball.position.copy(ballBody.position);
  ball.rotation.setFromRotationMatrix(ballBody.quaternion);

  bat.position.copy(batBody.position);
  bat.rotation.setFromRotationMatrix(batBody.quaternion);

  renderer.render(scene, camera);
}

animate();
