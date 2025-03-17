use futures_util::StreamExt;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use url::Url;

#[derive(Debug)]
pub struct WebSocketHttpClient {
    server_url: String,
    message: Arc<Mutex<Option<String>>>,
}

impl WebSocketHttpClient {
    pub fn new(server_url: String) -> Self {
        Self {
            server_url,
            message: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn request(&mut self, query_params: &serde_json::Value) -> Result<serde_json::Value, serde_json::Value> {
        let ws_url = format!("{}?{}", self.server_url, self.encode_query_params(query_params));
        let url: Url = Url::parse(&ws_url).unwrap();
        let (ws_stream, _) = connect_async(url.to_string()).await.unwrap();
        let (_write, mut read) = ws_stream.split();

        let message = self.message.clone();

        let response = tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        *message.lock().unwrap() = Some(text);
                    }
                    Ok(Message::Close(Some(CloseFrame { code, reason }))) => {
                        match code {
                            CloseCode::Library(4390) => {
                                return Ok(json!({ "status": 200, "message": message.lock().unwrap().take().unwrap_or_default() }));
                            }
                            CloseCode::Library(4490) => {
                                return Err(json!({ "status": 400, "message": reason }));
                            }
                            _ => {}
                        }
                        return Err(json!({ "status": 400, "message": "WebSocket connection closed unexpectedly" }));
                    }
                    _ => {}
                }
            }
            // Default response if no message is received
            Err(json!({ "status": 400, "message": "No message received" }))
        }).await.unwrap(); // Unwrap the JoinHandle result

        response
    }

    fn encode_query_params(&self, data: &serde_json::Value) -> String {
        data.as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| {
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    _ => value.to_string(), // Convert non-string values to string representation
                };
                format!("{}={}", urlencoding::encode(key), urlencoding::encode(&value_str))
            })
            .collect::<Vec<_>>()
            .join("&")
    }
}


/*
# Here the how server send message

// 1) connection creation
query pram read by server.


// 2) send message from server
ws.send("message");


// 3) closed connection by server after send message
ws.close(4390, 'success');
ws.close(4490, 'failed');



the client wait for closed connection of websocket from server then create response based on :
1) on receive it store the message in variable for future use like : this.message = message
2) on connection closed means the server is respond full then client integrate and create response by :
  if close-code 4390 => response {  status: 200 , message: this.message } ,
  if close-code 4490 => response {  status: 400 , message: 'failed' } 

  
usage:
    let mut client = WebSocketHttpClient::new("ws://ip:port".to_string());
    let query_params = json!({
        "uid": "InternalServerServicesCall",
        "callType": "seasonReset"
    });

    match client.request(&query_params).await {
        Ok(response) => println!("Success: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
*/
