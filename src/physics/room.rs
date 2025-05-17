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
    room_type: String,
    room_id: Uuid,
    room_capacity: i32,
    room_free: bool,
    players_in_room: Vec<Uuid>,
    phys_world: Arc<Mutex<PhysicsWorld>>,

}

impl Room {
    pub async fn new() -> Self  {
        let room_type = "TBC";
        let room_id = Uuid::new_v4(); //Creating room_id.
        let room_capacity = 2;

        let physics_world = Arc::new(Mutex::new(PhysicsWorld::new())); //Creating async phys world, so can be accessed safely across threads.


        Room { //Init Room.
            room_type,
            room_id,
            room_capacity,
            true,
            players_in_room: Vec::new(),
            physics_world,
        }
        
    }

    pub fn get_room_id(&mut self) -> Uuid {
        self.room_id
    }

    pub fn add_player(&mut self,player_id: Uuid) {
        //Add player to room logic.
        //Will add player to world.



    }

    
}