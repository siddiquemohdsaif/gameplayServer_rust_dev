use crate::{game_data::Game, game_server::Message, utils::db_handler};
use rand::Rng;
use serde_json::json;
const BOT_UIDS: [&str; 100] = ["POu7aEDcwYCunEgr","ZIijciwKy2k7QiQP","wlqGnGVrN8tIaery","veLkYw5NIk6pXNm1","VDGdV1yGbRmHIMpR","Rv9ARTWFLNrPOozG","1YQcSuZ6v9MzAAFq","KGCxejtHfyLJdFBz","2XBoO864JEfQZAxC","4D4zBABasqomLND7","jzNihL9cZPOvWnkT","FdYVVpMGO5q8oYhh","JuhOumtDZjUAMmUn","udzQDiEl2ddL7aFw","gZ2iGmhV8IWQe1oe","irr77Cx0NInH3w3t","ZmrjXQAft9UMpUyB","UjTeJQ4ghufofw9x","hhY83hd5yboAMvMj","Mbrnw7ohcoG6o9og","mMuer1utA9LotkD8","L9O6ffAhPaQ1REQq","dLEAzzpZiRIEGAfX","HVJ4Ow4mdLzDjCg4","VsRwlNeUoelAjvkH","kZ6lECU5nJ1KztNV","A4JbIxslsNUi8Ggx","pKVM8YjS91dETVVl","MJbhXLXkeY50Rmp3","l05tD5Y6xhMvm3s0","zJzwXeEoMxbymR85","PihGdykQYK9AbS7Z","2SYNLN7EqLWfZlMK","UWBrBSWIkkC3Brxc","TJFpCWdDuQg8v5BO","c3wRBMB3rqCZZ2nQ","Kd2mMif1ArTYwaEX","K3wR78CzmsydD6Hn","nQfy07j7o1ZtzaSX","zleSikkpC2x8ir9o","xIhPF4UvyxBK8zBy","FPsIPHadtl7EeRgy","IveH6ZbSgDhJjbg5","03P3voEg7LNxWLBp","yHv52TPAOHrn6l4B","euDN322sZHr3yGhR","4MiLfAxTMt6iSvrZ","pbJ1oaqEczJvn12g","GGGQulQPHrMsUakh","EegrVPPKVgbmWbnc","sA80RwKCCW9tF8Px","x4hPDDnoThUwnLUU","DroSqWWIqTPyPnZj","41XKlSE5UsUHEmcs","sY25Jo0rfuK9Hzye","pcAiVNTJNTLoRf74","h7i1RTzQv8zxgoNi","ctiYK8DgRv023aws","WDwD1stOWwG13IwN","cBHrUTT3BchLvjBF","Mkz6Aai7hX6WMVth","oon0ZvLAjCJNFx5E","vu85RhjFI5PepLpE","iIxcn0EKZkDzDS3v","xhBcHxBuTJbS9qqq","ySSkTINISUjVWXD7","yeR78OP58Dz21YjK","DmtaxLFgQfxXsycS","PSw8lcWtzNYE79rC","4Uof42oF3SxzyRP2","tviUHoNYVXTBgMKg","vrNuhOHZN6pG9bFu","iWH0yOkFLEuWOpHO","4vNPKmUc5h0KuDGH","EDDsMCNTTKn3qBJJ","8UTjRg0RmchmFSww","RzrzTi3kQC8WZUvI","kcXChG8jIKci2LQy","RDwoan4E7htsG4OH","2P9gptu62VKgFLSe","d6wZGoNgQ8UGtonW","Bv1cJ669n80EvYlA","ZMd47alihN4RZQoL","IZDJAjoJxjDRcULC","mN1ViuVc0xT7j4lu","UvHC2MBPNq5x73GN","qXfdxtvOTc1rWsnO","413fadLXhrJphttB","f8IPqD2a9iKJXy5H","4rxYbx3cXvQR9mlH","3lyR1xWL1kGGChR6","QwYVT0PaSHD7E9mh","e6Sm3YVPkOUWzXKk","gcqRWvWrcrPwSz0d","biQOoUVohe57sedu","LIYDDtjyjnNuvMk4","dnEiBdBVgX3sexry","6H8BhboqTrKbesMU","wT936ZibKmbaVrrl","LK78YpwY9ZJzJjpF"];

fn random_between(start: i32, end: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..=end)
}


pub fn game_end_due_to_wating(game: &Game) {
    game_end_due_to_player_disconnect(game); // same as disconnect logic
}


pub fn game_end_due_to_player_disconnect(game: &Game) {
    let p1 = game.player_1_socket.clone(); // game state is ON no worry of null
    let p2 = game.player_2_socket.clone(); // game state is ON no worry of null

    let bots_uid = &BOT_UIDS;

    //bot dont lose game if loose connection hence example if player1(bot) disconnected and player2(non bot) connected still player2 lose and bot win 

    // Check if players are bots
    let p1_is_bot = bots_uid.contains(&game.uid_1.as_str());
    let p2_is_bot = bots_uid.contains(&game.uid_2.as_str());

    let winner = if p1.is_some() {
        if p2_is_bot {
            1 // p2 is disconnect but its bot hence win
            
        }else{
            0 // p1 is connected hence win
        }

    }else if p2.is_some() {
        if p1_is_bot {
            0 // p1 is disconnect but its bot hence win
            
        }else{
            1 // p2 is connected hence win
        }
    }else{
        let winner = random_between(0,1); // no one is connected chose randomly
        winner
    };

    let message = serde_json::to_string(&json!({ "type": "gameOver", "winner": winner })).unwrap();
    
    if p1.is_some() {
        Message::send_text(&p1.unwrap(), message.clone());
    }
    if p2.is_some() {
        Message::send_text(&p2.unwrap(), message);
    }
    db_handler::game_over_event_handle_async_task(game.uid_1.clone(),game.uid_2.clone(), game.map, winner, game.game_state.clone(),game.game_state_history.clone(),game.game_id.clone())
}


pub fn game_end_due_to_multiple_times_player_disconnect(game: &Game, uid: String) {
    let p1 = game.player_1_socket.clone(); // game state is ON no worry of null
    let p2 = game.player_2_socket.clone(); // game state is ON no worry of null

    let winner = if uid == game.uid_2 {
        0 // p1 is win because p2 diconnect mult times
    } else {
        1 // p2 is win because p1 diconnect mult times
    };

    let message = serde_json::to_string(&json!({ "type": "gameOver", "winner": winner })).unwrap();
    
    if p1.is_some() {
        Message::send_text(&p1.unwrap(), message.clone());
    }
    if p2.is_some() {
        Message::send_text(&p2.unwrap(), message);
    }
    db_handler::game_over_event_handle_async_task(game.uid_1.clone(),game.uid_2.clone(), game.map, winner, game.game_state.clone(),game.game_state_history.clone(),game.game_id.clone())
}


pub fn game_end_due_game_time_limit(game: &Game) {
    let p1 = game.player_1_socket.clone().unwrap(); // game state is ON no worry of null
    let p2 = game.player_2_socket.clone().unwrap(); // game state is ON no worry of null

    let winner = random_between(0,1);
    let message = serde_json::to_string(&json!({ "type": "gameOver", "winner": winner })).unwrap();
    Message::send_text(&p1, message.clone());
    Message::send_text(&p2, message);
    db_handler::game_over_event_handle_async_task(game.uid_1.clone(),game.uid_2.clone(), game.map, winner, game.game_state.clone(),game.game_state_history.clone(),game.game_id.clone())
}



pub fn game_end_due_to_player_win(winner: i32, game: &Game) {
    game.print_game_state_history();
    db_handler::game_over_event_handle_async_task(game.uid_1.clone(),game.uid_2.clone(), game.map, winner, game.game_state.clone(),game.game_state_history.clone(),game.game_id.clone())
}
