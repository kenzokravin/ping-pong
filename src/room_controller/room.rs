//This file is the "Room" struct.
//It holds all the necessary data that a room requires. 
//A room is a lobby of players, or their "world".
//It is used to isolate each physics world to it's own instance.

use uuid::Uuid;

 mod physics_world; //importing code from physics_world.
 use physics_world::PhysicsWorld;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::Player;

pub struct Room {
    pub room_type: String,
    pub id: Uuid,
    pub capacity: i32,
    pub pop: i32,
    pub is_free: bool,
    pub state: String,
    pub players_in_room: Vec<Player>,
    pub physics_world: Arc<Mutex<PhysicsWorld>>,

}

impl Room {
    pub async fn new() -> Self  {
        let room_type = "TBC";
        let id = Uuid::new_v4(); //Creating room_id.
        let capacity = 2;
        let state = "Not Started";
        let is_free = true;
        let pop = 0;

        let physics_world = Arc::new(Mutex::new(PhysicsWorld::new())); //Creating async phys world, so can be accessed safely across threads.


        Room { //Init Room.
            room_type:room_type.to_string(),
            id,
            capacity,
            pop,
            is_free,
            state:state.to_string(),
            players_in_room: Vec::new(),
            physics_world,
        }
        
    }

    pub fn get_room_id(&mut self) -> Uuid {
        self.id
    }

    pub fn add_player(&mut self,player: Player) {
        //Add player to room logic.
        //Will add player to world.
        self.players_in_room.push(player);
        self.pop += 1;



    }
    
    pub fn start_room(&mut self) {

    }

    pub fn end_room(&mut self) {
        //Disconnect all players from room.

    }

    pub fn tick_room(&mut self, dt:f32) {

    }
    
}