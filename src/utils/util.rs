use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn compare_state_string(json_string1: &str, json_string2: &str) -> bool {
    let obj1: Value = serde_json::from_str(json_string1).unwrap_or(Value::Null);
    let obj2: Value = serde_json::from_str(json_string2).unwrap_or(Value::Null);

    if obj1 == Value::Null || obj2 == Value::Null {
        return false;
    }

    if obj1["playerTurn"] != obj2["playerTurn"]
        || obj1["strikerId_1"] != obj2["strikerId_1"]
        || obj1["strikerId_2"] != obj2["strikerId_2"]
        || obj1["playerExtraInfo1"] != obj2["playerExtraInfo1"]
        || obj1["playerExtraInfo2"] != obj2["playerExtraInfo2"]
        || obj1["isOnlyQueenPocketedLast"] != obj2["isOnlyQueenPocketedLast"]
    {
        return false;
    }

    let binding = vec![];
    let carroms1_array = obj1["carroms"].as_array().unwrap_or(&binding);
    let binding = vec![];
    let carroms2_array = obj2["carroms"].as_array().unwrap_or(&binding);

    let carroms1: Vec<&Value> = carroms1_array.iter().filter(|carrom| carrom["isPotted"] == 0).collect();
    let carroms2: Vec<&Value> = carroms2_array.iter().filter(|carrom| carrom["isPotted"] == 0).collect();


    if carroms1.len() != carroms2.len() {
        return false;
    }

    for carrom1 in carroms1 {
        let carrom2 = carroms2
            .iter()
            .find(|&&c| c["coinCode"] == carrom1["coinCode"]);
        if carrom2.is_none() {
            return false;
        }
        let carrom2 = carrom2.unwrap();
        if carrom1["carrom_drawable_id"] != carrom2["carrom_drawable_id"]
            || carrom1["x"] != carrom2["x"]
            || carrom1["y"] != carrom2["y"]
            || carrom1["type"] != carrom2["type"]
        {
            return false;
        }
    }

    true
}

pub fn _is_valid_game_id(game_id: &str) -> bool {
    if game_id.len() != 32 {
        return false;
    }

    let uid1 = &game_id[0..16];
    let uid2 = &game_id[16..32];
    let timestamp_str = &game_id[32..];
    let timestamp = timestamp_str.parse::<u64>().unwrap_or(0);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let uid_re = regex::Regex::new(r"^[0-9a-zA-Z]{16}$").unwrap();
    if !uid_re.is_match(uid1) || !uid_re.is_match(uid2) {
        return false;
    }

    if timestamp < now - 3600000 || timestamp > now { // 1hr valid
        return true; // temp - true for debug
    }

    true
}


pub fn get_uids_and_timestamp_from_game_id(game_id: &str) -> Result<(String, String, u64), String> {
    if game_id.len() < 32 {
        return Err("length invalid".to_string());
    }

    let uid1 = game_id[0..16].to_string();
    let uid2 = game_id[16..32].to_string();
    let timestamp_str = &game_id[32..];
    let timestamp = timestamp_str.parse::<u64>().unwrap_or(0);

    let uid_re = regex::Regex::new(r"^[0-9a-zA-Z]{16}$").unwrap();
    if !uid_re.is_match(&uid1) || !uid_re.is_match(&uid2) {
        return Err("uid type invalid".to_string());
    }

    Ok((uid1, uid2, timestamp))
}



/*

mod utils;


fn main() {
    // Test the functions here
    let json1 = r#"{"playerTurn":1,"strikerId_1":2,"strikerId_2":3,"playerExtraInfo1":4,"playerExtraInfo2":5,"isOnlyQueenPocketedLast":false,"carroms":[{"coinCode":"A","isPotted":0,"carrom_drawable_id":0,"x":2,"y":3,"type":4}]}"#;
    let json2 = r#"{"playerTurn":1,"strikerId_1":2,"strikerId_2":3,"playerExtraInfo1":4,"playerExtraInfo2":5,"isOnlyQueenPocketedLast":false,"carroms":[{"coinCode":"A","isPotted":0,"carrom_drawable_id":0,"type":4,"x":2,"y":3}]}"#;
    println!("Compare State String: {}", utils::util::compare_state_string(json1, json2));
    println!("Is Valid Game ID: {:?}", utils::util::get_uids_and_timestamp_from_game_id("UMYCoJ8pTNHmC6iIcXOaHzKDZd0dyTPP1718722389027").unwrap());
}

*/