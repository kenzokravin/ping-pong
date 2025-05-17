//This file is the "Room Controller".
//It holds all the necessary data to manage room sessions on the server. 
//It will control which player is put in what room, when to start a session, how players join etc.


use std::collections::HashMap;
use uuid::Uuid;

mod room;
use room::Room;

pub struct RoomController {
    rooms: HashMap<Uuid,Room>,
}

impl RoomController {
    pub fn new(duration_secs: u64) -> Self  {

        RoomController { //Instantiating constructor variables.
            rooms: HashMap::new(),
        }
        
    }

    pub fn create_room(&mut self) -> Uuid { ... }

    pub fn join_room(&mut self, room_id: Uuid, player: Player) -> Result<(), Error> { ... }

    pub fn handle_input(&mut self, room_id: Uuid, input: PlayerInput) { ... }

    pub fn delete_room(&mut self, room_id: Uuid) {


    }

}