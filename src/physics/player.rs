//This file is the "Player"
//This will hold player data.
//It can also be used for database entries.
//These will be components such as ELO and display name.

use std::collections::HashMap;
use uuid::Uuid;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Player {
    id: Uuid,
    display_name: String,

}

impl Player {
    pub fn new() -> Self  {

        let id = Uuid::new_v4(); //Creating id.
        let display_name = "TBC";
        


        Player { //Init Room.
            id,
            display_name,
        }
        
    }

    pub fn get_id(&mut self) -> Uuid {
        self.id
    }

    
}