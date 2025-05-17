//To run rust local
//run "cargo run"
//open client.


use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::sync::broadcast;
use tokio::time::{self, Duration};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;
use rapier3d::prelude::*;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use std::collections::HashMap;

mod physics; //importing all code in /physics
use physics::PhysicsWorld;
use serde_json::Value;
use futures::{sink::SinkExt, stream::StreamExt};


#[tokio::main]
async fn main() {

    println!("Main Fn Started in main.rs");
    


    //'::' is a path seperator, used to access items (functions,structs,traits,constants etc)
    //within modules,types or namespaces. Very similar to the dot access in js/py.
    // It is used on types to call something on the type itself, not an instance.
    //In modules (above), it is used to access items such as structs at the path (e.g. std -> sync -> Arc)
    //Dot access (.) is used for instance-level method calls. 

    // Create the physics world
     let physics_world = Arc::new(Mutex::new(PhysicsWorld::new()));
     // Code explained:
     //PhysicsWorld::new() == struct constructor for a new instance of physics world.
     //Mutex::new == ensures only one thread can modify or read the physics world at any time.
     // This is important as it is a shared mutable (editable) environment.
     //Arc::new == is a thread-safe reference counter. Means that it can allow multiple threads or
     //async tasks to share ownership of the same data. Without arc, data could not pass between 
     //threads as Rust's ownership model prevents moving data across threads.
     //More on ARC. ARC means Atomic Reference Counter. It has a pointer on the stack that points to the heap value and also an atomic integer. This atomic integer
     // counts every instance of the variable across threads, when it hits 0, it is removed. This allows us to work across threads
     // as every thread sees the instantaneous value of the atomic integer (thus it can't be interfered incorrectly). Allowing multiple threads
     // to use the ARC variable.
     //MUTEX is a mutual exclusion primitive that allows only a singular thread to access the wrapped data at a time.


     //TL:DR, Rust prevents threads touching same data. Arc allows multiple to touch same data
     //Mutex ensures only 1 at a time. physics world is a readable/editable physics world.

    // Create a router with the WebSocket endpoint
    let (tx,_rx) = broadcast::channel(16); //creating channel
    //This essential means that the channel has a cap of 16 messages, this means that when 16 new messages come, the rest are dropped.
    //This is a trade off for performance and memory. Too low and less updates. Too high and too much memory.

    

    let tx_clone = tx.clone();
    //This creates a clone of the sender handle.

    let app = Router::new().route("/ws", get({
        let tx = tx.clone(); // clone here
        let physics_world = physics_world.clone();

        move |ws: WebSocketUpgrade| {
            let tx = tx.clone(); // clone again here if needed
            let rx = tx.subscribe();
            let physics_world = physics_world.clone();

            async move {
                ws.on_upgrade(move |socket| handle_socket(socket, physics_world, tx, rx))
            }
        }
    }));

     //This creates a new axum router and adds a new route "/ws"
     //the physics world is cloned so that this closure gets its own copy of Arc
     //necessary as the closure is move which takes ownership of the vars it uses, but we want
     //to keep the og physics_world alive elsewhere too.
     //Async move upgrades the http connection to ws.
     //move socket is an async closure that gets called with the new websocket connection.
     //inside this, we call handle_socket, passing it the socket and the cloned phys world.

     //closure is like a function that sees it's surrounding scope, so in this case, async move, can see
     //the physics world because it is a closure. (this being the move ws: websocket).

     //In this context, get is a func that takes a closure/func, defining how to respond to GET requests.
     //move tells Rust to move ownership of any used var into the closure.

    // Run the server
    // let addr = "127.0.0.1:3000".parse().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
     //IP is localhost using port 3000.
     // For LAN, change to "0.0.0.0:3000" as this is local network.
     // Other devices can join using local IP address like: 192.168.x.x:3000

     //For public hosting:
     // Change to 0 as well and ensure port 3000 is open in server firewall.
    

    //  println!("Running WebSocket server on ws://{}", addr);
    //  axum_server::bind(addr)
    //     .serve(app.into_make_service())
    //      .await
    //      .unwrap();

    //This server has to be async because of the .await and .serve
    //As servers run indefinitely, if it was not async then it blocks the rest of main().
    // tokio::spawn(async move {
    //     println!("Running WebSocket server on ws://{}", addr);
    //     axum_server::bind(addr)
    //         .serve(app.into_make_service())
    //         .await
    //         .unwrap();
    // });



    // println!("Running WebSocket server on ws://{}", addr);
    // axum_server::bind(addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    

     //Is the axum server being launched.
     // .parse() parses string into a socket address
     //axum::Server::bind, binds the server to the specific IP and port.
     //.serve(app.into_make_service()) , app is the router, this converts the router into a
     //format the server can understand for handling incoming http requests.
     //.await tells rust to wait asynchronously for the server to run.
     //It doesn't block the thread in synchronous code - other async tasks can still run.
     //.unwrap() , if something goes wrong, this will panic and print the error.

    //  let tx = tx.clone();
   
    let physics_world = physics_world.clone();

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(33)); //This decides tick rate (fps) (~30fps)
        //interval.tick().await;
   
        loop {
          
            interval.tick().await;
      
            //wait for the world to be freed up and to access.
            
                let mut world = physics_world.lock().await;
                world.step(1.0 / 30.0); // Advance physics
            

      

            // Extract ball position this is used to send state to client.
            if let Some(ball_body) = world.world.get(world.ball_handle) { // Some() is used if a val may or may not exist.
                //world.world.get is calling the physics world obj, then navigating to world in the struct, then using .get to search for the ball_handle (also a val in the struct)

                let position = ball_body.translation();
                let velocity = ball_body.linvel();


                let state = serde_json::json!({
                    "type": "ball_state",
                    "pos": [position.x, position.y, position.z],
                    "vel": [velocity.x, velocity.y, velocity.z]
                });

                //println!("Ball: {}",state);

                let _ = tx_clone.send(state.to_string()); // broadcast to clients
            }


            //Also need to send opposing player data here.

            let player_map = &world.player_map ;
            let mut player_index_num = 0;

            

            for (player_id, p_body_handle) in player_map { //Getting all players in hashmap.

                let player_body = world.world.get(*p_body_handle);

                let position = player_body.expect("No p_body").translation();

                let state = serde_json::json!({
                    
                    "type": "player_state", //Used to determine client-side action.
                    "player_id": player_id.to_string(),
                    "player_num": player_index_num, //Used to determine what "player number" the player is, to depict position in world space.
                    "pos": [position.x, position.y, position.z],         
                });


                let _ = tx_clone.send(state.to_string());

                player_index_num += 1;

            }

            }

        }
    );


    //Server is at bottom because it blocks the main() func from completing as .await and .serve are active indefinitely.
     println!("Running WebSocket server on ws://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}


async fn handle_socket(mut socket: WebSocket, physics_world: Arc<Mutex<PhysicsWorld>>, _tx:broadcast::Sender<String>, mut rx:broadcast::Receiver<String>) {

    //creating and assigning new playerID
     let player_id = Uuid::new_v4();

      // Add their paddle to the physics world
      // This code means that it waits until the thread gets exclusive access to physics_world
      // It then gives a mutable reference, so that the world.add_player() can be added.
      //without mutex (ensuring only 1 can access) then more than 1 player could alter the physics world at once.
    let mut player_index = -1;
    {
        let mut world = physics_world.lock().await;
        world.add_player(player_id);
        player_index = world.get_player_number(player_id);
    }

    println!("Player {} connected", player_id);

    let (mut sender, mut receiver) = socket.split();


    //Sending initial init message when user first connects to server.
    let welcome_msg = serde_json::json!({
            "type": "init", //sending message type for init.
            "player_id": player_id.to_string(), //sending player_id.
            "player_index": player_index, //returning player index to set order.
    });
    let _ = sender.send(Message::Text(welcome_msg.to_string().into())).await;

    // let mut send_socket = socket.copy();

    //This function creates a background async task that listens for messages on a channel and sends them over a websocket connect.
    

    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            //The following code converts the message into a websocket and attempts to send.
            if sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });

    

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {

                //println!("Received message: {}", text);

                //This area is where we handle player messages.
                //Here we will control the movement of the player bodies/colliders (their phys objects.)
                //We can also perform other stuff here (like send chat messages perhaps)
                 if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    if let Some(msg_type) = json.get("type").and_then(|v| v.as_str()) {

                        // Checks msg type is move so we know what vars are available.
                        if msg_type == "move" { 
                            let dx = json.get("dx").and_then(|val| val.as_f64());
                            let dy = json.get("dy").and_then(|val| val.as_f64());
                            let dz = json.get("dz").and_then(|val| val.as_f64());

                            {
                                let mut world = physics_world.lock().await;
                                world.add_move_to_queue(player_id,dx.expect("ERR: add_player_x is NONE"),
                                dy.expect("ERR: add_player_y is NONE"),dz.expect("ERR: add_player_z is NONE"));
                            }
                              
                        }

                        if msg_type == "hit_begin" {
                            //Triggering hit receive function.
                            {
                                let mut world = physics_world.lock().await;
                                world.player_hit(player_id);
                            }


                            println!("Received Hit Start for {}", player_id);



                        }

                        if msg_type == "hit_end" {

                             {
                                let mut world = physics_world.lock().await;
                                world.player_hit_exec(player_id).await;
                            }

                            //Triggering hit receive function.
                            println!("Received Hit Finish for {}", player_id);


                        }


                        //println!("msg value: {}", msg_type);
                    } else {
                        println!("'type' not found or not a float");
                    }
                } else {
                    println!("Failed to parse JSON");
                }
                
            }
            Message::Close(_) => {
                // Handle closing the WebSocket connection
                {
                    let mut world = physics_world.lock().await; //Waiting for thread to gain access to phys world.
                    world.remove_player(player_id); //Calls remove_player logic (in Physics World instance)
                }


                //Sending information to remove player when they disconnect.
                let removal_msg = serde_json::json!({
                    "type": "remove",
                    "player_id": player_id.to_string(),
                });

                // Send to all clients via broadcast channel
                if let Err(e) = _tx.send(removal_msg.to_string()) {
                    eprintln!("Broadcast error on player remove: {}", e);
                }

                println!("Connection closed");
                break;
            }

            _ => {} //This takes care of all unneeded message types such as Ping and Pong (used to ignore them).
        }
    }
}

// // Simple physics world struct to hold the rapier world
// //This is used to create our own world in the main() function.
// struct PhysicsWorld {
//     world: RigidBodySet,
//     colliders: ColliderSet,
//     gravity: Vector3<f32>,
//     player_map: HashMap<Uuid,RigidBodyHandle>,
//     collider_map: HashMap<ColliderHandle, Uuid>, //Reverse lookup for collision detect.
//     ball_handle: RigidBodyHandle,
// }

// impl PhysicsWorld {
//     fn new() -> Self {
//         let mut world = RigidBodySet::new(); //creating empty collections.
//         let mut colliders = ColliderSet::new();

//         // You could initialize the world with some simple objects, like paddles and ball
//         let ball = RigidBodyBuilder::new_dynamic()
//             .translation(Vector3::new(0.0, 0.0, 0.0))
//             .build();
//         let ball_collider = ColliderBuilder::ball(0.1).build();
//         colliders.insert(ball_collider);
//         let ball_handle = world.insert(ball);


//         //This is outputting the physics world as it's return variable using the struct.
//         PhysicsWorld {
//             world,
//             colliders,
//             gravity: Vector3::new(0.0, 0.0, 0.0), //No gravity as all the directions are controlled programatically.
//             //Could potentially add gravity later though.
//             player_map: HashMap::new(),
//             collider_map: HashMap::new(),
//             ball_handle,
//         }
//     }

//     fn step(&mut self, dt: f32) {
//         let gravity = self.gravity;
//         let integration_parameters = IntegrationParameters {
//             dt: dt,
//             ..Default::default()
//         };
//         let mut solver = ImpulseSolver::new();
//         self.world.step(&gravity, &integration_parameters, &mut solver);
//     }

//     fn add_player(&mut self, player_id: Uuid)   {
//         //&mut self just allows the function to reference it's var (in this case being physics world)
//         //This allows us to make changes to the collections such as world, and colliders etc
//         //player_id is just the unique player id and it's type.

//         let player_body = RigidBodyBuilder::new_kinematic_position_based()
//             .translation(vector![0.0, 0.0, 0.0]) // Starting position
//             .build();
//         let player_body_handle = self.world.insert(player_body);

//         let player_collider = ColliderBuilder::cylinder(0.05, 0.5).build();
//         self.colliders.insert_with_parent(player_collider, player_body_handle, &mut self.world);


//         //tracking player bodies
//         self.player_map.insert(player_id,player_body_handle);
//         //tracking player colliders
//         self.collider_map.insert(player_collider,player_id);

//         // You could store paddle_handle in a map if you want to track per-player paddles
//     }

// }

