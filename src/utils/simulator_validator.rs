use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::game_data::PreviousGameState;

#[derive(Serialize)]
#[allow(non_snake_case)]
struct RequestBody {
    gameStateSting: String,
    simulatorEventString: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct ResponseBody {
    endGameState: String,
}

pub async fn do_simulation_validation(previous_state: PreviousGameState, current_simulation: String) -> Result<String, Box<dyn Error>> {

    let client = Client::new();

    let request_body = RequestBody {
        gameStateSting: previous_state.state.clone(),
        simulatorEventString: current_simulation.clone(),
    };

    let response = client.post("http://localhost:9097/simulate")
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;
    let result: ResponseBody = serde_json::from_str(&response_text)?;

    Ok(result.endGameState)
}




// #[derive(Clone)]
// struct GameState {
//     state: String,
// }

// #[derive(Clone)]
// struct Game {
//     previous_state: GameState,
//     current_simulation: String,
//     state: String,
// }

// fn main() {
//     let runtime = Runtime::new().unwrap();

//     let game = Arc::new(Mutex::new(Game {
//         previous_state: GameState {
//             state: "previous_state_string".to_string(),
//         },
//         current_simulation: "current_simulation_string".to_string(),
//         state: "current_state_string".to_string(),
//     }));

//     let result = runtime.block_on(do_simulation_validation("game_event", game.clone()));

//     match result {
//         Ok(state) => println!("Final game state: {}", state),
//         Err(e) => println!("Error during simulation validation: {}", e),
//     }
// }
