use std::collections::HashMap;
use actix_web::web::Bytes;
use lazy_static::lazy_static;
use serde_json::Value;
use std::sync::Mutex;
use std::sync::Arc;
use actix_web_actors::ws;
use actix::Addr;
use crate::game_event;
use crate::game_over;
use crate::game_server::Action;
use crate::game_server::Message;
use crate::game_server::MyWebSocket;
use crate::utils::db_handler;
use crate::utils::signaling_handler;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use crate::utils::message_builder;
use serde::{Serialize, Deserialize};


const MAX_WAIT_TIME: i64 = 15_000;            // 15 seconds
const MAX_GAME_TIME: i64 = 1_800_000;         // 30 minutes

lazy_static! {
    pub static ref GAME_DATA: Arc<Mutex<GameData>> = Arc::new(Mutex::new(GameData::new()));
}

pub struct GameData{
    pub games: HashMap<String, Game>,
}


impl GameData {
    pub fn new() -> Self {
        GameData {
            games: HashMap::new(),
        }
    }
}


#[derive(Clone)]
pub struct Game {
    pub game_id: String,

    pub player_1_socket: Option<Addr<MyWebSocket>>,
    pub player_2_socket: Option<Addr<MyWebSocket>>,
    pub uid_1: String,
    pub uid_2: String,
    pub map: i32,
    pub game_state : String,
    pub previous_game_state: Option<PreviousGameState>,
    pub current_simulation: Option<String>,
    pub game_state_history: Vec<GameStateHistory>, 

    pub st: i64,                                                 // game start time
    pub cst: i64,                                                // current turn start time
    pub dt: i64,                                                 // any one player disconnect or game pause time
    pub ttp : i32,                                               // total time passed in current turn. it gone adding if in one turn miltiple pasuse resume happen to calculate exact time passed.
    pub state: State,

    pub pending_fire_game_event: Vec<String>,                    // if sate pause but other player just manage to fire gameEvents so, on gave resume it send to other player.
    pub p1_disconnect_count: i32,  
    pub p2_disconnect_count: i32, 

    pub grtc_connection_id: String,                                  // id to sync
    pub grtc_signaling_failed_count: i32,                         // max number of retry allowed to build grtc connection
    pub grtc_connection_state: GRTCConnectionState,               // is grtc connected

}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GRTCConnectionState {
    DISCONNECTED,
    CONNECTED_ONE_SIDE,
    CONNECTED_BOTH_SIDE
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum State {
    ON,
    WAITING,
    PAUSE,
    END,
}

#[derive(Clone)]
pub struct PreviousGameState {
    pub state: String,
    pub update_count: i32,
    pub game_event: String,
    pub at: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameStateHistory {
    pub id: String,
    pub game_state: String,
}

impl Game {
    pub fn new(game_id: String, uid_1: String, uid_2: String, game_state: String, map: i32) -> Self {
        let now = Utc::now().timestamp_millis();
        let initial_history = GameStateHistory {
            id: "Start".to_string(),  // Convert to String
            game_state: game_state.clone(), // Clone the game_state for history
        };
        
        Game {
            game_id,
            player_1_socket: None,
            player_2_socket: None,
            uid_1,
            uid_2,
            map,
            game_state,
            previous_game_state: None,
            current_simulation: None,
            game_state_history: vec![initial_history],
            cst: now,
            st: now,
            dt: now,                           //disconnected time is now because no player is connected yet, all not connected in limited time game will end and winner will be connected one or random if no one connected.
            ttp : 0,
            state: State::WAITING,
            pending_fire_game_event: Vec::new(),
            p1_disconnect_count: 0,
            p2_disconnect_count: 0,

            grtc_connection_id: "NULL".to_string(),
            grtc_signaling_failed_count: 0,
            grtc_connection_state: GRTCConnectionState::DISCONNECTED,
        }
    }

    pub fn print_game_state_history(&self) {
        println!("Game State History:");
        for history in &self.game_state_history {
            println!("ID: {}, Game State: {}", history.id, history.game_state);
        }
    }

    pub fn send_message_to_player_1(&self, message: String) {
        if let Some(addr) = &self.player_1_socket {
            Message::send_text(addr, message);
        }
    }

    pub fn send_message_to_player_2(&self, message: String) {
        if let Some(addr) = &self.player_2_socket {
            Message::send_text(addr, message);
        }
    }

    pub fn _send_bin_message_to_player_1(&self, message: Bytes) {
        if let Some(addr) = &self.player_1_socket {
            Message::send_binary(addr, message);
        }
    }

    pub fn _send_bin_message_to_player_2(&self, message: Bytes) {
        if let Some(addr) = &self.player_2_socket {
            Message::send_binary(addr, message);
        }
    }

    fn sockets_exchange(&self) {
        let p1 = self.player_1_socket.clone().unwrap(); // game state is on no worry of null
        let p2 = self.player_2_socket.clone().unwrap(); // game state is on no worry of null
        Action::set_other_player_socket(&p1, p2.clone());
        Action::set_other_player_socket(&p2, p1);
    }

    pub fn exit_player_1(&self, code: ws::CloseCode, description: Option<String>) {
        if let Some(addr) = &self.player_1_socket {
            Action::exit(addr, code, description);
        }
    }

    pub fn exit_player_2(&self, code: ws::CloseCode, description: Option<String>) {
        if let Some(addr) = &self.player_2_socket {
            Action::exit(addr, code, description);
        }
    }

    fn player_connect(&mut self,  uid: String, socket: Addr<MyWebSocket>) -> bool{
        //start game if both player connected
        match self.state {
            State::ON => {
                return self.handle_at_game_onn_player_connect(uid, socket);
            },
            State::PAUSE => {
                return self.handle_at_game_pause_player_connect(uid, socket);
            },
            State::WAITING => {
                return self.handle_at_game_waiting_player_connect(uid, socket);
            },
            State::END => {
                return false;
            }
        }
    }

    fn player_disconnect(&mut self,  uid: String) {
        //pause game if state ON
        match self.state {
            State::ON => {
                self.handle_at_game_onn_player_diconnect(uid);
            },
            State::PAUSE => {
                self.handle_at_game_pause_player_diconnect(uid);
            },
            State::WAITING => {
                self.handle_at_game_waiting_player_diconnect(uid);
            },
            State::END => {
                return;
            }
        }
    }




    fn handle_at_game_waiting_player_connect(&mut self,  uid: String, socket: Addr<MyWebSocket>) -> bool{
        if self.uid_1 == uid {
            //connected as player 1

            if self.player_1_socket.is_some() {
                //player 1 is already there hence disconnect connected one and add this
                self.exit_player_1(ws::CloseCode::Normal, Some("description".into()));
            }

            //add player
            self.player_1_socket = Some(socket.clone());


        } else if self.uid_2 == uid {
            //connected as player 2

            if self.player_2_socket.is_some() {
                //player 2 is already there hence disconnect connected one and add this
                self.exit_player_2(ws::CloseCode::Normal, Some("description".into()));
            }

            //add player
            self.player_2_socket = Some(socket.clone());


        } else {
            //uid not found , put the game back and return false
            return false;
        }

        if self.player_1_socket.is_some() && self.player_2_socket.is_some() {
            self.start_game();
        }
        return true;
    }

    fn handle_at_game_pause_player_connect(&mut self,  uid: String, socket: Addr<MyWebSocket>) -> bool{
        if self.uid_1 == uid {
            //connected as player 1

            if self.player_1_socket.is_some() {
                //player 1 is already there hence disconnect connected one and add this
                self.exit_player_1(ws::CloseCode::Normal, Some("description".into()));
            }

            //add player
            self.player_1_socket = Some(socket.clone());

        } else if self.uid_2 == uid {
            //connected as player 2

            if self.player_2_socket.is_some() {
                //player 2 is already there hence disconnect connected one and add this
                self.exit_player_2(ws::CloseCode::Normal, Some("description".into()));
            }

            //add player
            self.player_2_socket = Some(socket.clone());


        } else {
            //uid not found , put the game back and return false
            return false;
        }

        if self.player_1_socket.is_some() && self.player_2_socket.is_some() {
            self.resume_game(uid);
        }else {
            if self.player_1_socket.is_some() {
                self.send_message_to_player_1(message_builder::get_game_state_msg(&self.game_state,self.ttp));
                self.send_message_to_player_1(message_builder::get_pause_msg());
            }
            if self.player_2_socket.is_some() {
                self.send_message_to_player_2(message_builder::get_game_state_msg(&self.game_state,self.ttp));
                self.send_message_to_player_2(message_builder::get_pause_msg());
            }
        }
        return true;
    }

    fn handle_at_game_onn_player_connect(&mut self,  uid: String, socket: Addr<MyWebSocket>) -> bool{
        if self.uid_1 == uid {
            //connected as player 1

            if self.player_1_socket.is_some() {
                //player 1 is already there hence disconnect connected one and add this
                self.exit_player_1(ws::CloseCode::Normal, Some("description".into()));
            }

            //add player
            self.player_1_socket = Some(socket.clone());


        } else if self.uid_2 == uid {
            //connected as player 2

            if self.player_2_socket.is_some() {
                //player 2 is already there hence disconnect connected one and add this
                self.exit_player_2(ws::CloseCode::Normal, Some("description".into()));
            }

            //add player
            self.player_2_socket = Some(socket.clone());


        } else {
            //uid not found , put the game back and return false
            return false;
        }

        return true;
    }




    fn handle_at_game_onn_player_diconnect(&mut self,  uid: String){
        if self.uid_1 == uid {
            //disconnected as player 1
            //remove player
            self.player_1_socket = None;

        } else if self.uid_2 == uid {
            //disconnected as player 2
            //remove player
            self.player_2_socket = None;
        }

        if self.player_1_socket.is_none() || self.player_2_socket.is_none() {
            self.pause_game();
        }
    }

    fn handle_at_game_waiting_player_diconnect(&mut self,  uid: String){
        if self.uid_1 == uid {
            //disconnected as player 1
            //remove player
            self.player_1_socket = None;

        } else if self.uid_2 == uid {
            //disconnected as player 2
            //remove player
            self.player_2_socket = None;
        }
    }

    fn handle_at_game_pause_player_diconnect(&mut self,  uid: String){
        if self.uid_1 == uid {
            //disconnected as player 1
            //remove player
            self.player_1_socket = None;

        } else if self.uid_2 == uid {
            //disconnected as player 2
            //remove player
            self.player_2_socket = None;
        }
    }




    fn start_game(&mut self){
        self.state = State::ON;
        self.sockets_exchange();
        self.cst = Utc::now().timestamp_millis();
        self.send_message_to_player_1(message_builder::get_game_state_msg(&self.game_state,0));
        self.send_message_to_player_2(message_builder::get_game_state_msg(&self.game_state,0));
    
        //grtc new start
        signaling_handler::start_new_grt_connection(self);
    
    }

    fn resume_game(&mut self, uid: String){

        self.state = State::ON;
        self.sockets_exchange();
        self.ttp = self.ttp + self.get_time_passed(); // total time passed in this turn already
        self.cst = Utc::now().timestamp_millis();


        //increase disconnect times
        if uid == self.uid_1 {
            self.p1_disconnect_count = self.p1_disconnect_count + 1;
        }else {
            self.p2_disconnect_count = self.p2_disconnect_count + 1;
        }

        if uid == self.uid_1 {
            self.send_message_to_player_1(message_builder::get_game_state_msg(&self.game_state,self.ttp)); //start game by send current game state
            self.send_message_to_player_2(message_builder::get_resume_msg(&self.game_state,self.ttp)); // resume
            game_event::complete_simulation_end_event(self, self.uid_2.clone()); // complete previous game state update count -> 2 if not
            game_event::fire_pending_events(self, self.uid_2.clone()); // complete all fired event of player 2 
        }else {
            self.send_message_to_player_1(message_builder::get_resume_msg(&self.game_state,self.ttp)); // resume
            self.send_message_to_player_2(message_builder::get_game_state_msg(&self.game_state,self.ttp)); //start game by send current game state
            game_event::complete_simulation_end_event(self, self.uid_1.clone());// complete previous game state update count -> 2 if not
            game_event::fire_pending_events(self, self.uid_1.clone());// complete all fired event of player 1
        }


        //grtc new start
        self.grtc_signaling_failed_count = 0; // reset fail count
        signaling_handler::start_new_grt_connection(self);


    }

    fn pause_game(&mut self){
        self.state = State::PAUSE;
        self.dt = Utc::now().timestamp_millis();
        self.send_message_to_player_1(message_builder::get_pause_msg());
        self.send_message_to_player_2(message_builder::get_pause_msg());

        //grtc stop
        signaling_handler::stop_grt_connection(self);
    }

    fn get_time_passed(&mut self) -> i32{
        let now  = Utc::now().timestamp_millis();
        let tp = (now - self.cst) - (now - self.dt);
        return tp as i32;
    }

    fn start_game_monitor(game_id: String) {
        tokio::spawn(async move {
            loop {
                let game_end = Game::process_game(&game_id);
                if game_end {
                    break;
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    fn process_game(game_id: &String) -> bool{

        let now  = Utc::now().timestamp_millis();

        let mut game_data = super::game_data::GAME_DATA.lock().unwrap(); 
        if let Some(game) = game_data.games.get(game_id) {
            match game.state {
                State::ON => {
                    //game over if 30 minutes over
                    if (game.st + MAX_GAME_TIME) < now {
                        //game end
                        game_over::game_end_due_game_time_limit(&game);
                        game_data.games.remove(game_id);
                        return true;
                    }

                    //game previous state check
                    if game.previous_game_state.is_some() {

                        let previous_game_state = game.previous_game_state.clone().unwrap();
                        if previous_game_state.at + 3000 < now && game.cst + 1000 < now {
                            game_event::complete_simulation_end_event(&game, game.uid_1.clone());
                        }
                        
                    }

                    //game disconnect count check
                    if game.p1_disconnect_count  >= 5 {
                        //game end
                        game_over::game_end_due_to_multiple_times_player_disconnect(&game, game.uid_1.clone());
                        game_data.games.remove(game_id);
                        return true;
                    }else if game.p2_disconnect_count  >= 5 {
                        //game end
                        game_over::game_end_due_to_multiple_times_player_disconnect(&game, game.uid_2.clone());
                        game_data.games.remove(game_id);
                        return true;
                    }

                    return false;
                },
                State::PAUSE => {
                    if (game.dt + MAX_WAIT_TIME) < now {
                        //game end
                        game_over::game_end_due_to_player_disconnect(&game);
                        game_data.games.remove(game_id);
                        return true;
                    }
                    return false;
                },
                State::WAITING => {
                    if (game.dt + MAX_WAIT_TIME) < now {
                        //game end
                        game_over::game_end_due_to_wating(&game);
                        game_data.games.remove(game_id);
                        return true;
                    }
                    return false;
                },
                State::END => {
                    return true;
                }
            };

        } else {
            return true;
        }
    }

}


pub fn _is_game_found (game_id: &String) -> bool {
    let game_data = GAME_DATA.lock().unwrap(); 
    if let Some(_game) = game_data.games.get(game_id) {
       return true;
    } else {
        return false;
    }
}


pub fn init_new_game (game_id: String, game_state: String, uid_1: String, uid_2: String, ip: String) -> bool {
    let mut game_data = GAME_DATA.lock().unwrap();
    if let Some(_game) = game_data.games.get(&game_id) {
        return false; // game found
    } else {
        let parsed: Value = serde_json::from_str(&game_state).unwrap_or_default();
        let map = parsed["map"].as_i64().unwrap_or(-404) as i32;
        if map == -404 {
            //error
            return false;
        }
        let game = Game::new(game_id.clone(), uid_1.clone(), uid_2.clone(), game_state, map);
        game_data.games.insert(game_id.clone(), game);
        Game::start_game_monitor(game_id.clone()); //temp
        db_handler::game_start_event_handle_async_task(uid_1, uid_2, map, game_id, ip);
        return true;
    }
}


pub fn connect_player_to_game(game_id: String, uid: String, socket: Addr<MyWebSocket>) -> bool {
    let mut game_data = GAME_DATA.lock().unwrap();
    if let Some(mut game) = game_data.games.remove(&game_id) {      
        let is_connected =  game.player_connect(uid, socket);
        game_data.games.insert(game_id, game);
        return is_connected;
    } else {
        return false;
    }
}

pub fn disconnect_player_from_game(game_id: String, uid: String){
    let mut game_data = GAME_DATA.lock().unwrap();
    if let Some(mut game) = game_data.games.remove(&game_id) {  
        game.player_disconnect(uid);
        game_data.games.insert(game_id, game);
    }
}