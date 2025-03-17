use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct ReceivedMessage {
    #[serde(rename = "type")]
    pub _type: String,
    pub text: Option<String>,
    pub message: Option<String>,
    pub state: Option<String>,
    pub event: Option<String>,
}

impl ReceivedMessage {
    // Function to create an instance of ReceivedMessage from a JSON string
    pub fn from(json_str: &str) -> Result<Self> {
        serde_json::from_str(json_str)
    }

    // Function to convert an instance of ReceivedMessage to a JSON string
    pub fn _to(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}