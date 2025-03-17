use crate::{game_data::{self, GRTCConnectionState, Game, State}, game_server::Message};

use serde::{Serialize, Deserialize};
use serde_json::json;



#[derive(Serialize, Deserialize)]
pub struct FAILED {
    pub code: i32,
    pub msg: String,
}



#[derive(Serialize, Deserialize)]
pub struct SignalingMessage {
    #[serde(rename = "type")]
    pub _type: String,
    pub connection_id: String,
}


pub fn handle_signaling_message(game_id: String, uid: String, msg_data: String) {

    let signaling_message: SignalingMessage = match serde_json::from_str(&msg_data) {
        Ok(rec_msg) => rec_msg,
        Err(_) => return
    };



    let mut game_data = game_data::GAME_DATA.lock().unwrap();
    if let Some(mut game) = game_data.games.remove(&game_id) {

        if game.state == State::ON && game.grtc_connection_id == signaling_message.connection_id {

            match signaling_message._type.as_str() {
                "failed" => {
                    //decode:
                    let _failed_info: FAILED = match serde_json::from_str(&msg_data) {
                        Ok(rec_msg) => rec_msg,
                        Err(_) => return
                    };
                    //restart if try is less then 3 else failed
                    if game.grtc_signaling_failed_count >= 3 {
                        // permanentaly failed not retry further
                        failed_to_create_grt_connection(&mut game);
                        stop_grt_connection(&mut game);

                    }else{
                        // next retry
                        game.grtc_signaling_failed_count = game.grtc_signaling_failed_count + 1;
                        start_new_grt_connection(&mut game);
                    }
                }
                "offer" | "answer" | "candidate" => {

                    if game.grtc_connection_state != GRTCConnectionState::CONNECTED_BOTH_SIDE {
                        // proxy signaling message to other player
                        let message = serde_json::to_string(&json!({ "type": "signaling", "message": msg_data })).unwrap();
                        if uid == game.uid_1 {
                            let p2 = game.player_2_socket.clone().unwrap(); // game state is on no worry of null
                            Message::send_text(&p2, message);

                        }else{
                            let p1 = game.player_1_socket.clone().unwrap(); // game state is on no worry of null
                            Message::send_text(&p1, message);
                        }
                    }

                }
                "connected" => {

                    if game.grtc_connection_state == GRTCConnectionState::DISCONNECTED {
                        game.grtc_connection_state = GRTCConnectionState::CONNECTED_ONE_SIDE;
                    
                    }else if game.grtc_connection_state == GRTCConnectionState::CONNECTED_ONE_SIDE {
                        game.grtc_connection_state = GRTCConnectionState::CONNECTED_BOTH_SIDE;
                        // send connected commang to both player to build grtc connection
                        let message = serde_json::to_string(&json!({ "type": "signaling", "message": "{ \"type\": \"build_grtc_connection\" }" })).unwrap();
                        let p1 = game.player_1_socket.clone().unwrap(); // game state is on no worry of null
                        let p2 = game.player_2_socket.clone().unwrap(); // game state is on no worry of null
                        Message::send_text(&p1, message.clone());
                        Message::send_text(&p2, message);
                    }

                }
                "grtc_connection_brake" => {
                    if game.grtc_connection_state != GRTCConnectionState::DISCONNECTED {
                        game.grtc_connection_state = GRTCConnectionState::DISCONNECTED;
                        if game.grtc_signaling_failed_count < 3 {
                            start_new_grt_connection(&mut game);
                        }
                    }
                }
                t => {
                    eprintln!("Unknown signaling message type:{}",t);
                }
            }

        }

        
        if game.state != State::END {
            game_data.games.insert(game_id, game); //put return back game not over yet
        }
    }
    
    

}


fn failed_to_create_grt_connection(game: &mut Game){

    game.grtc_connection_state = GRTCConnectionState::DISCONNECTED;

    //start/restart siganaling process
    let p1 = game.player_1_socket.clone();
    let p2 = game.player_2_socket.clone();

    let connection_id = game.grtc_connection_id.clone();
    let message_data_obj = json!({
        "type": "failed",
        "connection_id": connection_id,
        "error": "max(3) retry reached!",
    });
    let message_data = serde_json::to_string(&message_data_obj).unwrap();
    let message = serde_json::to_string(&json!({ "type": "signaling", "message": message_data })).unwrap();
        
    if p1.is_some() {
        Message::send_text(&p1.unwrap(), message.clone());
    }

    if p2.is_some() {
        Message::send_text(&p2.unwrap(), message);
    }
} 

fn generate_random_id(length: usize) -> String {
    use rand::{distributions::Alphanumeric, Rng};
    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    id
}


pub fn start_new_grt_connection(game: &mut Game){

    if game.state == State::ON {
        game.grtc_connection_state = GRTCConnectionState::DISCONNECTED;

        //start/restart siganaling process
        let p1 = game.player_1_socket.clone().unwrap(); // game state is on no worry of null
        let p2 = game.player_2_socket.clone().unwrap(); // game state is on no worry of null

        let connection_id = generate_random_id(8);
        let message_data_obj = json!({
            "type": "start_new_grtc_connection",
            "connection_id": connection_id
        });
        let message_data = serde_json::to_string(&message_data_obj).unwrap();
        let message = serde_json::to_string(&json!({ "type": "signaling", "message": message_data })).unwrap();
        
        game.grtc_connection_id = connection_id;
        Message::send_text(&p1, message.clone());
        Message::send_text(&p2, message);
    }

} 



pub fn stop_grt_connection(game: &mut Game){

    game.grtc_connection_state = GRTCConnectionState::DISCONNECTED;

    //start/restart siganaling process
    let p1 = game.player_1_socket.clone();
    let p2 = game.player_2_socket.clone();
    
    let connection_id = game.grtc_connection_id.clone();
    let message_data_obj = json!({
        "type": "stop_grtc_connection",
        "connection_id": connection_id
    });
    let message_data = serde_json::to_string(&message_data_obj).unwrap();
    let message = serde_json::to_string(&json!({ "type": "signaling", "message": message_data })).unwrap();
    
        
    if p1.is_some() {
        Message::send_text(&p1.unwrap(), message.clone());
    }

    if p2.is_some() {
        Message::send_text(&p2.unwrap(), message);
    }

} 





























