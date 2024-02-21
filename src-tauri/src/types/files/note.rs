use super::FileSerializable;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    body: String,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            body: String::new(),
        }
    }
}

impl FileSerializable for Note {
    fn custom_deserialize(file_content: &[u8]) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            body: String::from_utf8_lossy(file_content).to_string(),
        }))
    }

    fn custom_serialize(&self) -> Result<Vec<u8>> {
        Ok(self.body.as_bytes().to_vec())
    }
}
