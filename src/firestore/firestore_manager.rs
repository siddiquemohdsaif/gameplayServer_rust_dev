use reqwest::{Client, ClientBuilder, header::{HeaderMap, HeaderName, HeaderValue}};
use serde_json::{json, Value};
use std::sync::Once;
use crate::firestore::validator::{validate_collection_name, validate_document_name};

#[derive(Debug)]
pub struct FirestoreManager {
    client: Client,
    base_url: String,
}

static mut INSTANCE: Option<FirestoreManager> = None;
static INIT: Once = Once::new();

const APP_URL: &str = "https://carrom-clash-9t32.cloudsw3.com/rest-api/";
const AUTHORIZATION_TOKEN: &str = "a6MjKPcgU9XhLR1N";

impl FirestoreManager {
    pub fn get_instance() -> &'static FirestoreManager {
        unsafe {
            INIT.call_once(|| {
                let mut headers = HeaderMap::new();
                headers.insert(
                    HeaderName::from_static("authorization"),
                    HeaderValue::from_str(&format!("Bearer {}", AUTHORIZATION_TOKEN)).unwrap(),
                );

                let client = ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .expect("Failed to build client");

                let instance = FirestoreManager {
                    client,
                    base_url: APP_URL.to_string(),
                };
                INSTANCE = Some(instance);
            });
            INSTANCE.as_ref().expect("Instance not initialized")
        }
    }

    pub async fn create_document(
        &self,
        coll_name: &str,
        doc_name: &str,
        parent_path: &str,
        mut document: Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        validate_collection_name(Some(coll_name))?;
        validate_document_name(Some(doc_name))?;

        if document.get("_id").is_some() {
            return Err("document should not have '_id' field.".into());
        }

        document["_id"] = json!(doc_name);
        let body = serde_json::to_string(&document)?;

        let url = format!(
            "{}cr?collName={}&parentPath={}",
            self.base_url, coll_name, parent_path
        );

        let response = self
            .client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response)
    }

    pub async fn read_document(
        &self,
        coll_name: &str,
        doc_name: &str,
        parent_path: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        validate_collection_name(Some(coll_name))?;
        validate_document_name(Some(doc_name))?;

        let url = format!(
            "{}rd?parentPath={}&collName={}&docName={}",
            self.base_url, parent_path, coll_name, doc_name
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let json_response = response.json::<Value>().await?;
            Ok(json_response)
        } else {
            let text_response = response.text().await?;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                text_response,
            )))
        }
    }



    pub async fn _update_document(
        &self,
        coll_name: &str,
        doc_name: &str,
        parent_path: &str,
        mut document: Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        validate_collection_name(Some(coll_name))?;
        validate_document_name(Some(doc_name))?;

        if document.get("_id").is_some() {
            return Err("document should not have '_id' field.".into());
        }

        document["_id"] = json!(doc_name);
        let body = serde_json::to_string(&document)?;

        let url = format!(
            "{}upd?collName={}&parentPath={}",
            self.base_url, coll_name, parent_path
        );

        let response = self
            .client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response)
    }

    pub async fn delete_document(
        &self,
        coll_name: &str,
        doc_name: &str,
        parent_path: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        validate_collection_name(Some(coll_name))?;
        validate_document_name(Some(doc_name))?;

        let url = format!(
            "{}deld?collName={}&docName={}&parentPath={}",
            self.base_url, coll_name, doc_name, parent_path
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let json_response = response.json::<Value>().await?;
            Ok(json_response)
        } else {
            let text_response = response.text().await?;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                text_response,
            )))
        }
    }
    
}
