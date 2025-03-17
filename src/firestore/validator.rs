use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ValidationError {}

pub fn validate_document_name(document_name: Option<&str>) -> Result<(), ValidationError> {
    match document_name {
        None => Err(ValidationError {
            message: String::from("document name is null"),
        }),
        Some(name) if name.is_empty() => Err(ValidationError {
            message: String::from("document name is empty"),
        }),
        Some(name) if name.contains('`') => Err(ValidationError {
            message: String::from("document name has a restricted character (`)"),
        }),
        Some(name) if name.contains('/') => Err(ValidationError {
            message: String::from("document name has a restricted character (/)"),
        }),
        Some(name) if name.contains(' ') => Err(ValidationError {
            message: String::from("document name has a restricted character (space)"),
        }),
        Some(_) => Ok(()),
    }
}

pub fn validate_collection_name(collection_name: Option<&str>) -> Result<(), ValidationError> {
    match collection_name {
        None => Err(ValidationError {
            message: String::from("collection name is null"),
        }),
        Some(name) if name.is_empty() => Err(ValidationError {
            message: String::from("collection name is empty"),
        }),
        Some(name) if name.contains('`') => Err(ValidationError {
            message: String::from("collection name has a restricted character (`)"),
        }),
        Some(name) if name.contains('/') => Err(ValidationError {
            message: String::from("collection name has a restricted character (/)"),
        }),
        Some(name) if name.contains(' ') => Err(ValidationError {
            message: String::from("collection name has a restricted character (space)"),
        }),
        Some(_) => Ok(()),
    }
}
