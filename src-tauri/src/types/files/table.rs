use super::FileSerializable;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table;

impl Default for Table {
    fn default() -> Self {
        Self
    }
}

impl FileSerializable for Table {
    fn custom_deserialize(file_content: &[u8]) -> Result<Box<Self>> {
        unimplemented!()
    }

    fn custom_serialize(&self) -> Result<Vec<u8>> {
        unimplemented!()
    }
}
