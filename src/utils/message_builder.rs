use serde_json::json;

pub fn get_ping(ping: i64) -> String {

    let doc_msg = json!({
        "type": "ping",
        "ping" : ping
    });

    return serde_json::to_string(&doc_msg).unwrap();
}

pub fn get_game_state_msg(game_state: &str, tp: i32) -> String {

    let doc_msg = json!({
        "type": "gameState",
        "state": game_state,
        "TP" : tp
    });

    return serde_json::to_string(&doc_msg).unwrap();
}


pub fn  get_pause_msg() -> String {

    let doc_msg = json!({
        "type": "gameEvent",
        "event": {
            "eventType": "pause"
        }
    });

    return serde_json::to_string(&doc_msg).unwrap();
}


pub fn  get_resume_msg(game_state: &str, ttp: i32) -> String {

    let doc_msg = json!({
        "type": "gameEvent",
        "event": {
            "eventType": "resume",
            "ttp" : ttp,
            "state": game_state
        }
    });

    return serde_json::to_string(&doc_msg).unwrap();
}