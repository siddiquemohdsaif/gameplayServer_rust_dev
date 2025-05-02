use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{firestore::firestore_manager::FirestoreManager, game_data::GameStateHistory, utils::{backbone_server_url::BackboneServerUrl, websocket_http_client::WebSocketHttpClient}};


const ENV_CONFIG_JSON: &str = include_str!("../env_config.json");

#[derive(Debug, Deserialize)]
struct EnvConfig {
    #[serde(rename = "PRODUCTION_TYPE")]
    production_type: String
}

fn load_env_config() -> EnvConfig {
    serde_json::from_str::<EnvConfig>(ENV_CONFIG_JSON)
        .unwrap_or_else(|e| panic!("Invalid embedded JSON config: {}", e))
}

// on game-start
async fn create_reconnect_link(uid1: &String, uid2: &String, game_id: &String, ip: &String) -> Result<(), Box<dyn Error>> {
    
    let firestore_manager = FirestoreManager::get_instance();
    let reconnect_doc = json!({"gameId": game_id, "IP": ip});
    //set for uid1
    match firestore_manager.create_document("reconnectServerLink", &uid1, "/", reconnect_doc.clone()).await {
        Ok(_response) => {},
        Err(e) => eprintln!("Failed to create reconnectServerLink: {:?}", e),
    }
    //set for uid2
    match firestore_manager.create_document("reconnectServerLink", &uid2, "/", reconnect_doc).await {
        Ok(_response) => {},
        Err(e) => eprintln!("Failed to create reconnectServerLink: {:?}", e),
    }
    Ok(())

}
async fn deduct_coin_by_api_call(uid1: &String, uid2: &String, map: i32) -> Result<(), Box<dyn Error>> {
    
    let env_config = load_env_config();
    let base_api = if env_config.production_type == "release" {
        "https://function.cloudsw3.com/cc-app-api"
    } else {
        "https://function.cloudsw3.com/cc-app-api_dev"
    };
    
    let client = Client::new();
    let response = client.post(format!("{}/gamePlayServer/game-start", base_api))
        .json(&json!({"UID1": uid1, "UID2": uid2, "map": map}))
        .send()
        .await?;

    let _body = response.text().await?;
    Ok(())
}
async fn game_start_event_handle(uid1: String, uid2: String, map: i32, game_id: String, ip: String) -> Result<(), Box<dyn Error>>{

    if map == -1 {
        create_reconnect_link(&uid1, &uid2, &game_id, &ip).await?;
    } else {
        deduct_coin_by_api_call(&uid1, &uid2, map).await?;
        create_reconnect_link(&uid1, &uid2, &game_id, &ip).await?;
    }
    Ok(())
}


async fn upload_game_state_history(game_id: String, game_state_history: Vec<GameStateHistory>) -> Result<(), Box<dyn Error>> {
    // Get the singleton instance of FirestoreManager
    let firestore_manager = FirestoreManager::get_instance();

    // Serialize the game_state_history and prepare the document
    let reconnect_doc = json!({"game_state_history": game_state_history});

    // Attempt to create the document in Firestore
    match firestore_manager.create_document("MatchGameState", &game_id, "/Data/Surveillance/", reconnect_doc.clone()).await {
        Ok(_) => {
            // If the document creation is successful, do nothing further
        },
        Err(e) => eprintln!("Failed to upload_game_state_history: {:?}", e),
    }

    // Return Ok if everything goes well
    Ok(())
}












// on game-over
#[derive(Serialize, Deserialize)]
struct TrophyData {
    win: i32,
    lose: i32,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct PlayerExtraInfo {
    trophyData: TrophyData,
    isPlayerInWar: bool,
}
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct GameState {
    playerExtraInfo1: String,
    playerExtraInfo2: String,
}
async fn delete_reconnect_link(uid1: &String, uid2: &String) {
    
    let firestore_manager = FirestoreManager::get_instance();
    //set for uid1
    match firestore_manager.delete_document("reconnectServerLink", &uid1, "/").await {
        Ok(_response) => {},
        Err(e) => eprintln!("Failed to delete reconnectServerLink: {:?}", e),
    }
    //set for uid2
    match firestore_manager.delete_document("reconnectServerLink", &uid2, "/").await {
        Ok(_response) => {},
        Err(e) => eprintln!("Failed to delete reconnectServerLink: {:?}", e),
    }
}
async fn send_friendly_battle_result_to_bbs(uid1: &String, uid2: &String, winner: i32) {
    // Logic to call the backbone server to send a friendly-battle-result message card to all clan members
    let server_url_manager = BackboneServerUrl::new();
    let url = server_url_manager.get_url().await;
    let mut client = WebSocketHttpClient::new(url);
    let query_params = json!({
        "uid": "InternalServerServicesCall",
        "callType": "friendlyBattleResult",
        "UID1": uid1,
        "UID2": uid2,
        "winner": winner
    });

    match client.request(&query_params).await {
        Ok(_response) => {},
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
async fn game_over_event_handle(uid1: String, uid2: String, map: i32, winner: i32, game_state: String,game_state_history: Vec<GameStateHistory>,game_id: String) -> Result<(), Box<dyn Error>> {
    delete_reconnect_link(&uid1, &uid2).await;
    let _ = upload_game_state_history(game_id, game_state_history).await;


    let env_config = load_env_config();
    let base_api = if env_config.production_type == "release" {
        "https://function.cloudsw3.com/cc-app-api"
    } else {
        "https://function.cloudsw3.com/cc-app-api_dev"
    };

    if map == -1 {
        send_friendly_battle_result_to_bbs(&uid1, &uid2, winner).await;
    } else {
        let game_state: GameState = serde_json::from_str(&game_state)?;
        let player_extra_info1: PlayerExtraInfo = serde_json::from_str(&game_state.playerExtraInfo1)?;
        let player_extra_info2: PlayerExtraInfo = serde_json::from_str(&game_state.playerExtraInfo2)?;
        let data = json!({
            "UID1": uid1,
            "UID2": uid2,
            "winner": winner,
            "map": map,
            "p1TrophyWin": player_extra_info1.trophyData.win,
            "p1TrophyLose": player_extra_info1.trophyData.lose,
            "p2TrophyWin": player_extra_info2.trophyData.win,
            "p2TrophyLose": player_extra_info2.trophyData.lose,
            "isPlayer1InWar": player_extra_info1.isPlayerInWar,
            "isPlayer2InWar": player_extra_info2.isPlayerInWar,
            "playerExtraInfo1" : game_state.playerExtraInfo1,
            "playerExtraInfo2" : game_state.playerExtraInfo2,
        });

        let client = Client::new();
        let response = client.post(format!("{}/gamePlayServer/game-over", base_api))
            .json(&data)
            .send()
            .await?;

        let _body = response.text().await?;
    }
    Ok(())
}











pub fn game_start_event_handle_async_task(uid1: String, uid2: String, map: i32, game_id: String, ip: String) {
    tokio::spawn(async move {
        match game_start_event_handle(uid1, uid2, map, game_id, ip).await {
            Ok(_) => {},
            Err(e) => {eprintln!("error game start :{}", e)}
        }
    });
}


pub fn game_over_event_handle_async_task(uid1: String, uid2: String, map: i32, winner: i32, game_state: String, game_state_history: Vec<GameStateHistory>,game_id: String) {
    tokio::spawn(async move {
        match game_over_event_handle(uid1, uid2, map, winner, game_state,game_state_history,game_id).await {
            Ok(_) => {},
            Err(e) => {eprintln!("error game over :{}", e)}
        }
    });
}












































