
use crate::firestore::firestore_manager::FirestoreManager;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BackboneServerUrl {
    backbone_server_url: Arc<Mutex<Option<ServerUrl>>>,
}

struct ServerUrl {
    url: String,
    expire_time: u64,
}

impl BackboneServerUrl {
    pub fn new() -> Self {
        Self {
            backbone_server_url: Arc::new(Mutex::new(None)),
        }
    }

    fn is_backbone_server_url_valid(&self) -> bool {
        let backbone_server_url = self.backbone_server_url.lock().unwrap();
        match &*backbone_server_url {
            Some(server_url) => server_url.expire_time > current_time_millis(),
            None => false,
        }
    }

    async fn load_backbone_server_url(&self) {
        let result = FirestoreManager::get_instance()
            .read_document("Data", "ServerConfig", "/")
            .await
            .expect("Failed to read document");

        let mut backbone_server_url = self.backbone_server_url.lock().unwrap();
        *backbone_server_url = Some(ServerUrl {
            url: result["BackboneServerUrl"].as_str().unwrap().to_string(),
            expire_time: current_time_millis() + 10000,
        });
    }

    pub async fn get_url(&self) -> String {
        if !self.is_backbone_server_url_valid() {
            self.load_backbone_server_url().await;
        }

        let backbone_server_url = self.backbone_server_url.lock().unwrap();
        backbone_server_url.as_ref().unwrap().url.clone()
    }
}

fn current_time_millis() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis() as u64
}

/*
usage :
    let server_url_manager = BackboneServerUrl::new();
    let url = server_url_manager.get_url().await;
    println!("Backbone Server URL: {}", url);
*/