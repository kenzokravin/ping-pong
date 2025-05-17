//This file is the "Room" struct.
//It holds all the necessary data that a room requires. 
//A room is a lobby of players, or their "world".
//It is used to isolate each physics world to it's own instance.


use std::collections::HashMap;
use uuid::Uuid;

mod mod;
use mod::PhysicsWorld;

pub struct Room {
    room_id: Uuid,
    players_in_room: Vec<Uuid>,
}

impl Room {
    pub fn new(duration_secs: u64) -> Self  {
        let room_id = Uuid::new_v4();


        Room {
            room_id,
            player_in_room: HashMap::new(),
            
        }
        
    }

    
}