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

use crate::Player;

pub struct Room {
    room_type: String,
    id: Uuid,
    capacity: i32,
    pop: i32,
    is_free: bool,
    state: String,
    players_in_room: Vec<Player>,
    physics_world: Arc<Mutex<PhysicsWorld>>,

}

impl Room {
    pub async fn new() -> Self  {
        let room_type = "TBC";
        let id = Uuid::new_v4(); //Creating room_id.
        let capacity = 2;
        let state = "Not Started";

        let physics_world = Arc::new(Mutex::new(PhysicsWorld::new())); //Creating async phys world, so can be accessed safely across threads.


        Room { //Init Room.
            room_type.to_string(),
            id,
            capacity,
            r#true,
            state.to_string(),
            players_in_room: Vec::new(),
            physics_world,
        }
        
    }

    pub fn get_room_id(&mut self) -> Uuid {
        self.room_id
    }

    pub fn add_player(&mut self,player: Player) {
        //Add player to room logic.
        //Will add player to world.
        players_in_room.insert(player);
        self.pop += 1;



    }
    
    pub fn start_room(&mut self) {

    }

    pub fn end_room(&mut self) {
        //Disconnect all players from room.

    }

    pub fn tick(&mut self, dt:f32) {

    }
    
}