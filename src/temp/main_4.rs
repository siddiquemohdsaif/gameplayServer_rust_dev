mod firestore;

use firestore::firestore_manager::FirestoreManager;
use firestore::validator;

use serde_json::json;
use tokio;

#[tokio::main]
async fn main() {
    // Initialize FirestoreManager instance
    let firestore_manager = FirestoreManager::get_instance();

    // Example serde_json::Value document
    let value_document = json!({
        "name": "Test Document 2",
        "content": {
            "a": 20,
            "b": 30
        }
    });

    // Create document using serde_json::Value
    match firestore_manager.create_document("reconnectServerLink", "test_doc_2", "/", value_document).await {
        Ok(response) => println!("Document created successfully: {:?}", response),
        Err(e) => eprintln!("Failed to create document: {:?}", e),
    }

    // Read document
    match firestore_manager.read_document("reconnectServerLink", "test_doc_2", "/").await {
        Ok(document) => println!("Document read successfully: {:?}", document),
        Err(e) => eprintln!("Failed to read document: {:?}", e),
    }

    // Example serde_json::Value document for update
    let updated_value_document = json!({
        "name": "Updated Test Document 2",
        "content": {
            "a": 50,
            "b": 60
        }
    });

    // Update document using serde_json::Value
    match firestore_manager.update_document("reconnectServerLink", "test_doc_2", "/", updated_value_document).await {
        Ok(response) => println!("Document updated successfully: {:?}", response),
        Err(e) => eprintln!("Failed to update document: {:?}", e),
    }

    // Delete document
    match firestore_manager.delete_document("reconnectServerLink", "test_doc_2", "/").await {
        Ok(response) => println!("Document deleted successfully: {:?}", response),
        Err(e) => eprintln!("Failed to delete document: {:?}", e),
    }
}
