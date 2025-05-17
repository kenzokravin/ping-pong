//This file is the "Room" struct.
//It holds all the necessary data that a room requires. 
//A room is a lobby of players, or their "world".
//It is used to isolate each physics world to it's own instance.

use std::collections::HashMap;
use uuid::Uuid;

mod physics_world; //importing code from physics_world.
use physics_world::PhysicsWorld;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Room {
    room_id: Uuid,
    players_in_room: Vec<Uuid>,
    phys_world: Arc<Mutex<PhysicsWorld>>,
}

impl Room {
    pub async fn new() -> Self  {
        let room_id = Uuid::new_v4(); //Creating room_id.

        let physics_world = Arc::new(Mutex::new(PhysicsWorld::new())); //Creating async phys world, so can be accessed safely across threads.


        Room { //Init Room.
            room_id,
            players_in_room: Vec::new(),
            physics_world,
        }
        
    }

    
}