use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct GameState {
    pub active: bool,
    pub some_data: String,
}

impl GameState {
    pub fn new() -> Self { //Constructor
        Self { active: false, some_data: String::new() }
    }

    pub async fn start_action(&mut self) {
        self.active = true;
        self.some_data = "Started".to_string();
        println!("Action started");
    }

    pub async fn end_action(&mut self) {
        if self.active {
            self.active = false;
            println!("Action ended with data: {}", self.some_data);
        } else {
            println!("No action active");
        }
    }
}

pub type SharedGameState = Arc<Mutex<GameState>>;
