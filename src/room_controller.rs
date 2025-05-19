//This file is the "Room Controller".
//It holds all the necessary data to manage room sessions on the server. 
//It will control which player is put in what room, when to start a session, how players join etc.

use std::collections::HashMap;
use uuid::Uuid;

mod room;
use room::Room;

use crate::Player;

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

    pub async fn create_room(&mut self) { 
        let new_room = Room::new(); //Creating new room.
        self.rooms_list.push(new_room.await);
        //self.rooms.insert(new_room.await.id,new_room.await);
        
     }

    pub fn add_player_to_room(&mut self, player: Player) {

        //Search through rooms with room_free == true.
        //Add player to room.
         for room in self.rooms_list.iter_mut() {
    let room_state = room.state.to_string();

    if room.state == "Not Started" {
        if room.pop < room.capacity {
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
        for room in &mut self.rooms_list {
            room.tick_room(dt); //Stepping physics world in room.
        }
    }

    

}