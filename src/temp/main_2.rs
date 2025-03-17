use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use chrono::Utc;

struct MyWebSocket {
    uid: String,
    last_ping: Arc<Mutex<i64>>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection established: {}", self.uid);
        let last_ping = self.last_ping.clone();
        let uid = self.uid.clone();

        ctx.run_interval(Duration::from_secs(1), move |_, ctx| {
            let last_ping = last_ping.lock().unwrap();
            let now = Utc::now().timestamp_millis();
            if now - *last_ping > 6000 {
                println!("Closing connection due to inactivity: {}", uid);
                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Error,
                    description: Some("Inactivity".into()),
                }));
                ctx.stop();
            }else {
                println!("Still connected: {} , lp:{} , now:{}", uid, last_ping , now);
            }
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket connection closed: {}", self.uid);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                let mut last_ping = self.last_ping.lock().unwrap();
                *last_ping = Utc::now().timestamp_millis();
                ctx.pong(&msg);
            },
            Ok(ws::Message::Text(text)) => {

                // Handle text message
                //println!("Received text message: {}", text);
                ctx.text(format!("Echo: {}", text));
            },
            Ok(ws::Message::Binary(bin)) => {

                // Handle binary message
                //println!("Received binary message: {:?}", bin);
                ctx.binary(bin);  // Echo the binary data back
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => (),
        }
    }
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> impl Responder {
    let uid = req
        .query_string()
        .split('&')
        .find_map(|param| {
            let mut split = param.split('=');
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                if key == "uid" {
                    return Some(value.to_string());
                }
            }
            None
        });

    if let Some(uid) = uid {
        ws::start(
            MyWebSocket {
                uid,
                last_ping: Arc::new(Mutex::new(Utc::now().timestamp_millis())),
            },
            &req,
            stream
        )
    } else {
        Ok(HttpResponse::BadRequest().body("Missing uid parameter"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/ws", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
