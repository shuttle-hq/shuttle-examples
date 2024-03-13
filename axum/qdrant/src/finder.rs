use qdrant_client::qdrant::{value::Kind, ScoredPoint};

use crate::contents::File;

pub trait Finder {
    fn find(&self, key: &str) -> Option<String>;
    fn get_contents(&self, result: &ScoredPoint) -> Option<String>;
}

impl Finder for Vec<File> {
    fn find(&self, key: &str) -> Option<String> {
        for file in self {
            if file.path == key {
                return Some(file.contents.clone());
            }
        }
        None
    }

    fn get_contents(&self, result: &ScoredPoint) -> Option<String> {
        let text = result.payload.get("id")?;
        let kind = text.kind.to_owned()?;
        if let Kind::StringValue(value) = kind {
            self.find(&value)
        } else {
            None
        }
    }
}
