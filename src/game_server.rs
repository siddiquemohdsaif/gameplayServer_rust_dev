use actix::prelude::*;
use actix_web::web::Bytes;
use actix_web::{web, HttpRequest, Responder, HttpResponse};
use actix_web_actors::ws;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use chrono::Utc;
use std::collections::HashMap;
use urlencoding::decode;

use crate::game_data::GRTCConnectionState;
use crate::utils::{message_builder, signaling_handler};
use crate::utils::system_info::SystemInfo;
use crate::{game_data, game_event, utils::message_parser::ReceivedMessage};

const BOT_UIDS: [&str; 100] = ["JstJo1Rg9TqVKRfr","5OemZNKqUwJmIw2p","fxahgJVrv7JNAWx3","sjTmqMgNpoLyYON4","Ycp2H5yK2TU2pw9p","9J715TIsye10990J","7BcO8HZ2Xh2smtY8","daOfeCC7LNnNbMGc","ACn3u1h2kz28Zfhu","mCdGSa25YVVzQdmd","Mifq1AB32P2fZmCg","8KFM0H9sCMVQQgfY","E1Dk1K2byEdCtrU5","62ZdXYNUM1pN7KMs","txP3tsZKyyDaR4jY","CsGBdIqOWAeq9JYy","OdvQwgOItZEG3EGu","R1dT0aMILQQ2W0lx","BXOFJCeXYzT74fWy","zppsbYyvbTfdgzST","qwPe6fB436LHsOnK","FXXL3vOSBcoohE7V","emZHCmMYaQnzgguE","3oVJqFENJ0lbyt9l","NoaxdSln5N4c7Bzo","iTlqbTsbuCUIbxf2","EQkDLIk2TUDU4Gg1","Jc8qgkeVc02W898t","IjOeb7x4Rl4ecy1k","dUKjwoJWQOyjmKZb","WVPWQeaYHmVH0bIW","oT2R5Zpx1VbhBlFC","HaU72rK5YHSZ1KWp","SqURkGU66vsETAGY","jR40RjhE9rS7Effj","etacEqqGxwdvKlMM","IiDjjQf3Se7vwmBG","CawROmo7c3K2KPhj","NEsMWviP3SOQSDgJ","PFKE6IBPPZi7vgeP","SNOqwWGKvil1zRNr","0qmfBmi7zqTFfl6K","mTkb44esvVEkmraR","w5uw2hkfSSTHqAFk","cQChagdf8e8N5AXh","nZXOHNjXH3ZVap9w","3gJ5ZVh3xAB4MdGo","ISEWLGLjkc736MSV","33MyJOdUlTe3bkD3","8VruaVaHqT59UVSD","rzb2yxzhrNP0hOKd","HaZXEI7mTfWfnXeg","QCzgmIZM4vobGXdj","g4SO4zktUtEj43o2","20S95QFwrxcb4j3C","8rsobdMQ2vTOmWY2","DkUglMQ2q3w1KKS0","hrAJmgfpo2IXB4Fn","5Kw88O8gBKi6W3gC","PhceiitwAVq6vL8B","6sGLv34i2cOLBqVJ","ljeFfKENQ8H2DPd5","D6aXoNzGlULdT3LA","LZkSlrheJ79GNn9r","z2dIIxpfIJ1h8MYt","TDPi3dRk30fMzOFc","SNyg1lRDjgLrNY2Y","xm7lgmvVWesiMjLJ","gRhbI0iHDVeupJaX","eHdlqdhg8t0jEsEa","6DN57BsBgo1o1GIB","UaBYhsalxDuvebZ6","wKMNcWDV2r20Reve","8MkO4VaXxZj2D70T","30ocoatADsSDwJov","x1JE4SruwYMZnkeX","3jT8mBrdIankRHcL","xDwtcrqw0SeH51j7","neRxxOq9VbtLpSd7","iNc6Y2udWvE7tEh0","yoiWJogp2mO7ahMb","mPnmCMhxOfyb0RoH","oLkScPR3tgZaPGxQ","WJBKA1dG6D64psPh","EQ4RsdgJStU9xfNK","AJJ1VH9p1aspzRPD","EThvWpQnQutiAd2V","cLriGvndP3Cj7Fww","fF0yyliZOIHDnqFP","GPRoJNSp4QgG5nCA","rHEdPIJuQBqTgRKv","BHQ52H0A1bE8i8bE","n5h5akiSLkAqbVuo","tdJ5sKDXvVzoBANP","rbWqLZY5xkdTImgU","7WR8HfHrHnUXq0f0","vKVIwioSubjTWHHT","TEHXCuTTaXeK6Tpm","B8F3qikNBZoLTkx0","21EQpZhWRxzTwqfd"];


#[allow(non_snake_case)]
pub struct MyWebSocket {
    gameID: String,
    gameStateParam: Option<String>,
    IP: Option<String>,
    uid: Option<String>,
    last_ping: Arc<Mutex<i64>>,

    other_player_socket:  Arc<Mutex<Option<Addr<MyWebSocket>>>>,
    is_kicked: bool,
}




#[derive(Message)]
#[rtype(result = "()")]
pub struct Message{
    send_type: MessageType,
    text: Option<String>,
    binary: Option<Bytes>
}
pub enum MessageType {
    Text,
    Binary,
}
impl Message {
    pub fn send_text(addr: &Addr<MyWebSocket>, msg: String){
        addr.do_send(Message{
            send_type : MessageType::Text,
            text : Some(msg),
            binary :None
        });
    }
    pub fn send_binary(addr: &Addr<MyWebSocket>, msg: Bytes){
        addr.do_send(Message{
            send_type : MessageType::Binary,
            text : None,
            binary :Some(msg)
        });
    } 
}
impl Handler<Message> for MyWebSocket {
    type Result = ();
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        match msg.send_type {
            MessageType::Text => ctx.text(msg.text.unwrap()),
            MessageType::Binary => ctx.binary(msg.binary.unwrap())
        }
    }
}



#[derive(Message)]
#[rtype(result = "()")]
pub struct Action {
    action_type: ActionType,
    exit_data: Option<ExitData>,
    other_player_socket_data: Option<OtherPlayerSocketData>,
}
#[allow(non_camel_case_types)]
pub enum ActionType {
    EXIT,
    SET_OTHER_PLAYER_SOCKET,
}
pub struct ExitData {
    code: ws::CloseCode,
    description: Option<String>
}
pub struct OtherPlayerSocketData {
    other_player_socket: Addr<MyWebSocket>
}
impl Action {
    pub fn set_other_player_socket(addr: &Addr<MyWebSocket>, other_player_socket: Addr<MyWebSocket>){
        addr.do_send(Action{
            action_type: ActionType::SET_OTHER_PLAYER_SOCKET,
            other_player_socket_data: Some(OtherPlayerSocketData{other_player_socket}),
            exit_data: None,
        });
    }
    pub fn exit(addr: &Addr<MyWebSocket>, code: ws::CloseCode, description: Option<String>){
        addr.do_send(Action{
            action_type: ActionType::EXIT,
            other_player_socket_data: None,
            exit_data: Some(ExitData{code,description}),
        });
    }
}
impl Handler<Action> for MyWebSocket {
    type Result = ();
    fn handle(&mut self, act: Action, ctx: &mut Self::Context) {
        match act.action_type {
            ActionType::EXIT => {
                let exit_data = act.exit_data.unwrap();
                self.is_kicked = true;
                ctx.close(Some(ws::CloseReason {
                    code: exit_data.code,
                    description: exit_data.description,
                }));
            },
            ActionType::SET_OTHER_PLAYER_SOCKET => {
                let mut other_player_socket_mutex = self.other_player_socket.lock().unwrap();
                *other_player_socket_mutex = Some(act.other_player_socket_data.unwrap().other_player_socket);
            }
        }
    }
}






impl MyWebSocket {

    fn handle_game_initializer(&self, ctx: &mut ws::WebsocketContext<Self>, game_id: String, game_state: String, ip: String) -> bool {
        
        // initialize game :
        match crate::utils::util::get_uids_and_timestamp_from_game_id(&game_id) {
            Ok(game_id_data) => {
                if  game_data::init_new_game(game_id, game_state, game_id_data.0, game_id_data.1, ip) {
                    //game initialize finish close socket
                    ctx.close(Some(ws::CloseReason {
                        code: ws::CloseCode::Normal,
                        description: Some("Game state Initialize sucessful.".into()),
                    }));
                    return true;
                }else {
                    //game initialize failed due to already init close the socket 
                    ctx.close(Some(ws::CloseReason {
                        code: ws::CloseCode::Normal,
                        description: Some(format!("Game state Initialize failed, already game initialized: {}",self.gameID)),
                    }));
                    return true;
                }
            },
            Err(e) => {
                //game initialize failed close the socket
                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Policy,
                    description: Some(format!("Game state Initialize failed due to gameId invalid:{} error:{}",&game_id,e)),
                }));
                return false;
            }
        }

    }

    fn handle_connection(&self, ctx: &mut ws::WebsocketContext<Self>, game_id: String, uid: String) -> bool {

        if game_data::connect_player_to_game(game_id, uid, ctx.address().clone()) {
            
            return true;

        }else{
            //game connect failed close the socket
            // ctx.close(Some(ws::CloseReason {
            //     code: ws::CloseCode::Policy,
            //     description: Some("Game State not defined!".into()),
            // }));
            return false;
        }
        
    }

    fn init_game(&self, ctx: &mut ws::WebsocketContext<Self>) -> bool {
        if self.gameStateParam.is_some() && self.IP.is_some() {
            return self.handle_game_initializer(ctx, self.gameID.clone(), self.gameStateParam.clone().unwrap(), self.IP.clone().unwrap());
        } else if self.uid.is_some() {
            return self.handle_connection(ctx, self.gameID.clone(), self.uid.clone().unwrap());
        } else {
            //game initialize failed close the socket
            ctx.close(Some(ws::CloseReason {
                code: ws::CloseCode::Policy,
                description: Some("Game state Initialize failed.".into()),
            }));
            return false;
        }
    }


    fn init_connection_monitor(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let last_ping = self.last_ping.clone();

        ctx.run_interval(Duration::from_secs(3), move |_, ctx| {
            let last_ping = last_ping.lock().unwrap();
            let now = Utc::now().timestamp_millis();
            if now - *last_ping > 6000 {
                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Error,
                    description: Some("Inactivity".into()),
                }));
                ctx.stop();
            }
        });
    }


    fn handle_message(&self, ctx: &mut ws::WebsocketContext<Self>, msg: String) {
        let message_received: ReceivedMessage = match ReceivedMessage::from(&msg) {
            Ok(rec_msg) => rec_msg,
            Err(_) => return
        };
        
        match message_received._type.as_str() {
            "ping" => {

                let now = Utc::now().timestamp_millis();
                let mut last_ping = self.last_ping.lock().unwrap();
                *last_ping = Utc::now().timestamp_millis();
                ctx.text(message_builder::get_ping(now));

            },
            "signaling" => {

                if message_received.message.is_some() {
                    signaling_handler::handle_signaling_message(self.gameID.clone(), self.uid.clone().unwrap(), message_received.message.unwrap());
                }
            
            },
            "chatMessage" | "dataLine" => {
                let other_player_socket = self.other_player_socket.lock().unwrap();
                if let Some(ref other_player) = *other_player_socket {
                    Message::send_text(other_player, msg);
                }
            },
            "gameEvent" => {
                if message_received.event.is_some() {
                    game_event::handle_game_event(message_received.event.unwrap(), self.gameID.clone(), self.uid.clone().unwrap());
                }
            },
            _ => {}
        }
    }

    fn handle_message_binary(&self, _ctx: &mut ws::WebsocketContext<Self>, bin: Bytes) {
        let other_player_socket = self.other_player_socket.lock().unwrap();
        if let Some(ref other_player) = *other_player_socket {
            Message::send_binary(other_player, bin);
        }
    }
    

}





impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

  
fn started(&mut self, ctx: &mut Self::Context) {
    if self.init_game(ctx) {
        if self.uid.is_some() {
            self.init_connection_monitor(ctx);
        }
    } else {
        println!("Game initialization failed, closing the socket");

        // Construct JSON response
        let json_response = json!({
            "type": "gameEnded",
            "message": "Game Already ended"
        }).to_string();

        ctx.text(json_response);

        ctx.close(Some(ws::CloseReason {
            code: ws::CloseCode::Normal,
            description: Some("Game state Initialize failed.".into()),
        }));
    }
}

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if self.uid.is_some() && !self.is_kicked{
            game_data::disconnect_player_from_game(self.gameID.clone(), self.uid.clone().unwrap());
        }
    }
    

}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(_msg)) => {
                let now = Utc::now().timestamp_millis();
                let mut last_ping = self.last_ping.lock().unwrap();
                *last_ping = Utc::now().timestamp_millis();
                ctx.pong(format!("{}",now).as_bytes());
            },
            Ok(ws::Message::Text(text)) => {
                self.handle_message(ctx, text.into());
            },
            Ok(ws::Message::Binary(bin)) => {
                self.handle_message_binary(ctx, bin);
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => (),
        }
    }
}

pub async fn new_connection(req: HttpRequest, stream: web::Payload) -> impl Responder {
    let query_string = req.query_string();
    let params: HashMap<_, _> = query_string
        .split('&')
        .filter_map(|param| {
            let mut split = param.split('=');
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                Some((key.to_string(), decode(value).unwrap_or_default().into()))
            } else {
                None
            }
        })
        .collect();

    let game_id = params.get("gameID").cloned();
    let game_state = params.get("gameState").cloned();
    let ip = params.get("IP").cloned();
    let uid = params.get("uid").cloned();

    if  game_id.is_some() {
        ws::start(
            MyWebSocket {
                gameID: game_id.unwrap(),
                gameStateParam: game_state,
                IP : ip,
                uid: uid,
                last_ping: Arc::new(Mutex::new(Utc::now().timestamp_millis())),
                other_player_socket: Arc::new(Mutex::new(None)),
                is_kicked: false
            },
            &req,
            stream,
        )
    } else {
        Ok(HttpResponse::BadRequest().body("Missing mandatory 'uid' parameter"))
    }
}





pub async fn health_check(req: HttpRequest, _stream: web::Payload) -> impl Responder {
    let mut sys_info: SystemInfo = SystemInfo::new();
    let cpu_usage = sys_info.get_cpu_usage_percentage();
    let ram_usage = sys_info.get_ram_usage_percentage();

    let show_all: bool = req.query_string().contains("show_all=true");

    let game_data = super::game_data::GAME_DATA.lock().unwrap(); 
    let games_running = game_data.games.len();


    if show_all {

        let mut p2p_connected = 0;
        let mut p2p_connecting = 0;
        let mut p2p_failed = 0;
        let mut bot_game = 0;
    
        for (_key, game) in game_data.games.iter() {
            if game.grtc_connection_state == GRTCConnectionState::CONNECTED_BOTH_SIDE {
                p2p_connected += 1;
    
            }else if game.grtc_signaling_failed_count >= 3 {
                p2p_failed += 1;
                
            }else{
                p2p_connecting += 1;
            }
    
            if BOT_UIDS.contains(&game.uid_1.as_str()) || BOT_UIDS.contains(&game.uid_2.as_str()) {
                bot_game += 1;
            }
        }
    
        // Format the response
        HttpResponse::Ok().json(json!({
            "status": "ok",
            "cpu_usage_percent": cpu_usage,
            "ram_usage_percent": ram_usage,
            "games_running": games_running,
            "p2p_info": {
                "p2p_connected" : p2p_connected,
                "p2p_connecting" : p2p_connecting,
                "p2p_failed" : p2p_failed,
            },
            "bot_game" : bot_game,
        }))




    }else{

        // Format the response
        HttpResponse::Ok().json(json!({
            "status": "ok",
            "cpu_usage_percent": cpu_usage,
            "ram_usage_percent": ram_usage,
            "games_running": games_running,
        }))
        
    }

}

