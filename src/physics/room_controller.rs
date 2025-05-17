//This file is the "Room Controller".
//It holds all the necessary data to manage room sessions on the server. 
//It will control which player is put in what room, when to start a session, how players join etc.


use std::collections::HashMap;
use uuid::Uuid;

mod room;
use room::Room;

pub struct RoomController {
    rooms: HashMap<Uuid,Room>,
    rooms_array: Vec<Room>,
}

impl RoomController {
    pub fn new() -> Self  {

        RoomController { //Instantiating constructor variables.
            rooms: HashMap::new(),
            rooms_array: Vec::new(),
        }
        
    }

    pub fn create_room(&mut self) -> Uuid { 
        let new_room = Room::new();
        self.rooms_array.insert(new_room);
        
     }

    pub fn join_room(&mut self, room_id: Uuid, player: Player) -> Result<(), Error> {

        //Search through rooms with room_free == true.
        //Add player to room.

        
        
    }

    pub fn handle_input(&mut self, room_id: Uuid, input: PlayerInput) { 
        
    }

    pub fn delete_room(&mut self, room_id: Uuid) {


    }

}