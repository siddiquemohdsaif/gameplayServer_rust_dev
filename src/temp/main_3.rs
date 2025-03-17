mod firestore;

use firestore::validator;

fn main() {
    // Example usage:
    match validator::validate_document_name(Some("validDocumentName")) {
        Ok(_) => println!("Document name is valid"),
        Err(e) => println!("Error: {}", e),
    }

    match validator::validate_collection_name(Some("invalid/Collection Name")) {
        Ok(_) => println!("Collection name is valid"),
        Err(e) => println!("Error: {}", e),
    }
}

