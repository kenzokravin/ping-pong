use serde::Deserialize;

mod player;
use player::Player;


#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PlayerMessage {
    #[serde(rename = "join_room")]
    JoinRoom {
        player_data: Player,
    },
    #[serde(rename = "move")]
    Move {
        
        dx: f64,
        dy: f64,
        dz: f64,
    },
    #[serde(rename = "hit_begin")]
    HitBegin {

    },
     #[serde(rename = "hit_end")]
    HitEnd {

    },

}