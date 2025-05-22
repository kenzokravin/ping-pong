//This file is the "Player"
//This will hold player data.
//It can also be used for database entries.
//These will be components such as ELO and display name.

//Also might be able to handle websocket connections here. (Might need to verify)
//It would use unbounded MPSC channels (tx.send etc)

use uuid::Uuid;

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: Uuid,
    pub display_name: String,

}

impl Player {
    pub fn new() -> Self  {

        let id = Uuid::new_v4(); //Creating id.
        let display_name = "TBC"; //Display name.
        


        Player { //Init Player
            id,
            display_name:display_name.to_string(),
        }
        
    }

    pub fn get_id(&mut self) -> Uuid { //Retreive player id.
        self.id
    }

    
}