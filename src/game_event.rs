use crate::{game_data::{self, Game, GameStateHistory, PreviousGameState, State}, game_over, game_server::Message, utils::{self, db_handler, simulator_validator, util::compare_state_string}};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GameEvent {
    pub eventType: String,
    pub eventAction: Option<String>,
    pub currentState: Option<String>,
    pub turn: Option<i32>,
}

pub fn handle_game_event(game_event_str: String, game_id: String, uid: String) {
    
    let game_event: GameEvent = serde_json::from_str(&game_event_str).unwrap_or_else(|e| GameEvent {
        eventType: e.to_string(),
        eventAction: None,
        currentState: None,
        turn: None,
    });

    let mut game_data = game_data::GAME_DATA.lock().unwrap();

    if let Some(mut game) = game_data.games.remove(&game_id) {

        if game.state == State::ON {

            match game_event.eventType.as_str() {
                "simulationStart" => {
                    send_game_event_to_other_player(&mut game, game_event_str, uid); // ok
                }
                "simulationEnd" => {
                    update_game_state(game_event, &mut game, game_event_str, uid);
                }
                "timeOut" => {
                    handle_time_out(game_event, &mut game, game_event_str, uid);
                }
                "leave" => {
                    handle_leave_game(game_event, &mut game, game_event_str, uid);
                }
                t => {
                    eprintln!("Unknown game event type:{}",t);
                }
            }
            
        }else if game.state == State::PAUSE {

            match game_event.eventType.as_str() {
                "simulationEnd" => {
                    update_game_state_half(game_event, &mut game, game_event_str.clone(), uid);
                }
                "timeOut" => {
                    game.pending_fire_game_event.push(game_event_str);
                }
                "leave" => {
                    game.pending_fire_game_event.push(game_event_str);
                }
                _ => {
                    eprintln!("Unknown game event type");
                }
            }
        }


        if game.state != State::END {
            game_data.games.insert(game_id, game); //put return back game not over yet
        }
    }
}


pub fn complete_simulation_end_event(game: &Game, uid: String){
    if game.previous_game_state.is_some() && game.previous_game_state.clone().unwrap().update_count == 1 {
        let simulation_end_event = game.previous_game_state.clone().unwrap().game_event;
        let game_id = game.game_id.clone();
        tokio::spawn(async move {
            handle_game_event(simulation_end_event, game_id, uid);
        });  
    }
}


pub fn fire_pending_events(game: &mut Game, uid: String){
    let pending_fire_game_event = game.pending_fire_game_event.clone();
    let game_id =  game.game_id.clone();
    game.pending_fire_game_event.clear();

    tokio::spawn(async move {                      //use async task to prevent deadlock on game_data
        for event in pending_fire_game_event {
            handle_game_event(event, game_id.clone(), uid.clone());
        }
    });
}


fn send_game_event_to_other_player(game: &mut Game, game_event_str: String, uid: String) {
    let other_player_socket = if uid == game.uid_1 {
        game.player_2_socket.clone().unwrap()
    } else {
        game.player_1_socket.clone().unwrap()
    };
    
    game.current_simulation = Some(game_event_str.clone());
    let msg = json!({ "type": "gameEvent", "event": game_event_str });
    let msg_str = serde_json::to_string(&msg).unwrap();
    Message::send_text(&other_player_socket, msg_str);
}




fn update_game_state(game_event: GameEvent, game: &mut Game, game_event_str: String, _uid: String) {
    if let Some(ref mut previous_state) = game.previous_game_state {
        if previous_state.update_count == 1 {

            //check is both player game_state match
            if utils::util::compare_state_string(&game.game_state, &game_event.currentState.clone().unwrap_or_default()){

                previous_state.update_count = 2;
                update_game_state_action(game_event, game, false);

            }else {
                

                // investicate hack game state via server simulation
                investigate_hack_via_simulation(game_event, game);

            }
            
            
        } else {
            let previous_game_state = PreviousGameState{
                state: game.game_state.clone(), 
                update_count: 1,
                game_event: game_event_str,
                at: Utc::now().timestamp_millis()
            };
            game.previous_game_state = Some(previous_game_state);
            game.game_state = game_event.currentState.unwrap_or_default();
            let history = GameStateHistory {
                id: "update_game_state 1".to_string(),  // Convert to String
                game_state: game.game_state.clone(), // Clone the game_state for history
            };
            
            game.game_state_history.push(history);

        }
    } else {
        let previous_game_state = PreviousGameState{
            state: game.game_state.clone(),
            update_count: 1,
            game_event: game_event_str,
            at: Utc::now().timestamp_millis()
        };
        game.previous_game_state = Some(previous_game_state);
        game.game_state = game_event.currentState.unwrap_or_default();
        let history = GameStateHistory {
            id: "update_game_state 2".to_string(),  // Convert to String
            game_state: game.game_state.clone(), // Clone the game_state for history
        };
        
        game.game_state_history.push(history);
    }
}

fn investigate_hack_via_simulation(game_event: GameEvent, game: &mut Game){

    let previous_state = game.previous_game_state.clone();
    let current_simulation = game.current_simulation.clone();
    let game_state = game.game_state.clone();
    let game_id = game.game_id.clone();

    tokio::spawn(async move {
        let final_game_state: String = if previous_state.is_some() && current_simulation.is_some() {
            let simulation_game_state = simulator_validator::do_simulation_validation(previous_state.unwrap(), current_simulation.unwrap()).await;
            let simulation_game_state = match simulation_game_state {
                Ok(result) => result,
                Err(_) => game_state,
            };
            simulation_game_state
        }else {
            game_state
        };


        let mut game_data: std::sync::MutexGuard<game_data::GameData> = game_data::GAME_DATA.lock().unwrap();

        if let Some(mut game) = game_data.games.remove(&game_id) {

            if game.state == State::ON {
                
                let is_state_same = compare_state_string(&game.game_state, &final_game_state);
                if !is_state_same {
                    game.game_state = final_game_state; 
                }

                update_game_state_action(game_event, &mut game, true);
                

                if game.state != State::END {
                    game_data.games.insert(game_id, game); //put return back game not over yet
                }
            }
            
        };
        
    });  

}

fn update_game_state_half(game_event: GameEvent, game: &mut Game, game_event_str: String, _uid: String) {
    let previous_game_state = PreviousGameState{
        state: game.game_state.clone(), 
        update_count: 1,
        game_event: game_event_str,
        at: Utc::now().timestamp_millis()
    };
    game.previous_game_state = Some(previous_game_state);
    game.game_state = game_event.currentState.unwrap_or_default();
    let history = GameStateHistory {
        id: "update_game_state_half".to_string(),  // Convert to String
        game_state: game.game_state.clone(), // Clone the game_state for history
    };
    
    game.game_state_history.push(history);
}



fn update_game_state_action(game_event: GameEvent, game: &mut Game, forced_state_change: bool) {

    let p1 = game.player_1_socket.clone().unwrap(); // game state is on no worry of null
    let p2 = game.player_2_socket.clone().unwrap(); // game state is on no worry of null

    match game_event.eventAction.unwrap_or_default().as_str() {
        
        "turnChange" => {
            let turn = game_event.turn.unwrap();
            let message = if forced_state_change {
                serde_json::to_string(&json!({ "type": "turnChange", "turn": turn, "changeState": game.game_state.clone() })).unwrap()
            } else {
                serde_json::to_string(&json!({ "type": "turnChange", "turn": turn, "changeState": game.game_state.clone() })).unwrap()
            };
            game.cst = Utc::now().timestamp_millis(); 
            game.ttp = 0;
            Message::send_text(&p1, message.clone());
            Message::send_text(&p2, message);
        },

        "win" => {
            let winner = game_event.turn.unwrap();
            let message = serde_json::to_string(&json!({ "type": "gameOver", "winner": winner })).unwrap();
            Message::send_text(&p1, message.clone());
            Message::send_text(&p2, message);
            game.state = State::END;
            
            //process game-over network call for coin exchange, profile update, etc
            game_over::game_end_due_to_player_win(winner, &game);

        },

        _ => {}
    }

}





//### security risk (no hack proof): temperory only ### => benifit : fast turn change
fn handle_time_out(game_event: GameEvent, game: &mut Game, game_event_str: String, _uid: String) {

    let p1 = game.player_1_socket.clone().unwrap(); // game state is on no worry of null
    let p2 = game.player_2_socket.clone().unwrap(); // game state is on no worry of null


    // update state fully // security risk (no hack proof): temperory only
    let previous_game_state = PreviousGameState{
        state: game.game_state.clone(),
        update_count: 2,
        game_event: game_event_str,
        at: Utc::now().timestamp_millis()
    };
    game.previous_game_state = Some(previous_game_state);
    game.game_state = game_event.currentState.unwrap_or_default();
    let history = GameStateHistory {
        id: "handle_time_out".to_string(),  // Convert to String
        game_state: game.game_state.clone(), // Clone the game_state for history
    };
    
    game.game_state_history.push(history);

    match game_event.eventAction.unwrap_or_default().as_str() {
        
        "0" => {
            let turn = game_event.turn.unwrap();
            let message = serde_json::to_string(&json!({ "type": "turnChange", "turn": turn })).unwrap();
            game.cst = Utc::now().timestamp_millis();
            game.ttp = 0;
            Message::send_text(&p1, message.clone());
            Message::send_text(&p2, message);
        },

        "1" => { // opponent call means main player not responde timely
            let turn = game_event.turn.unwrap();
            let message = serde_json::to_string(&json!({ "type": "turnChange", "turn": turn, "changeState": game.game_state.clone() })).unwrap();
            game.cst = Utc::now().timestamp_millis();
            game.ttp = 0;
            Message::send_text(&p1, message.clone());
            Message::send_text(&p2, message);
        },

        _ => {}
    }
}





fn handle_leave_game(_game_event: GameEvent, game: &mut Game, _game_event_str: String, uid: String) {

    let p1 = game.player_1_socket.clone().unwrap(); // game state is on no worry of null
    let p2 = game.player_2_socket.clone().unwrap(); // game state is on no worry of null


    let winner = if uid == game.uid_1 {1} else {0};
    
    let message = serde_json::to_string(&json!({ "type": "gameOver", "winner": winner })).unwrap();
    Message::send_text(&p1, message.clone());
    Message::send_text(&p2, message);
    game.state = State::END;
    
    //process game-over network call for coin exchange, profile update, etc
    db_handler::game_over_event_handle_async_task(game.uid_1.clone(), game.uid_2.clone(), game.map, winner, game.game_state.clone(),game.game_state_history.clone(),game.game_id.clone());
}










