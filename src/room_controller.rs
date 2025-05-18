//This file is the "Room Controller".
//It holds all the necessary data to manage room sessions on the server. 
//It will control which player is put in what room, when to start a session, how players join etc.


use std::collections::HashMap;
use uuid::Uuid;

mod room;
use room::Room;

pub struct RoomController {
    rooms: HashMap<Uuid,Room>,
    rooms_list: Vec<Room>,
}

impl RoomController {
    pub fn new() -> Self  {

        RoomController { //Instantiating constructor variables.
            rooms: HashMap::new(),
            rooms_list: Vec::new(),
        }
        
    }

    pub fn create_room(&mut self) -> Uuid { 
        let new_room = Room::new(); //Creating new room.
        self.rooms_list.insert(new_room);
        
     }

    pub fn add_player_to_room(&mut self, player: Player) -> Result<(), Error> {

        //Search through rooms with room_free == true.
        //Add player to room.
         for room in self.rooms_list.as_mut() {

            let room_state = room.state;

            if room.state == "Not Started" {   //Checking game hasn't started.
                if room.pop < room.capacity { // Checking the room is not full.
                    room.add_player(player);
                }
            }
        }
    }

    pub fn handle_input(&mut self, room_id: Uuid, input: PlayerInput) { 
        
    }

    pub fn delete_room(&mut self, room_id: Uuid) {


    }

    pub fn process_rooms(&mut self, dt: f32) {
        for room in self.rooms_list.as_mut() {
            room.tick(dt); //Stepping physics world in room.
        }
    }

    

}