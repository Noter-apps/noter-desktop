use super::FileSerializable;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image;

impl Image {
    pub fn new(content: &[u8]) -> Self {
        Self
    }
}

impl Default for Image {
    fn default() -> Self {
        Self
    }
}

impl FileSerializable for Image {
    fn custom_deserialize(file_content: &[u8]) -> Result<Box<Self>> {
        unimplemented!()
    }

    fn custom_serialize(&self) -> Result<Vec<u8>> {
        unimplemented!()
    }
}
