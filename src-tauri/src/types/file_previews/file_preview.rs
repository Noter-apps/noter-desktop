use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum FileType {
    Note,
    TodoList,
    Image,
    Table,
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            Self::Note => "md".to_string(),
            Self::TodoList => "todo.csv".to_string(),
            Self::Image => "png".to_string(),
            Self::Table => "csv".to_string(),
        }
    }
}

impl FileType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "md" => Some(Self::Note),
            "todo.csv" => Some(Self::TodoList),
            "png" => Some(Self::Image),
            "csv" => Some(Self::Table),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilePreview {
    pub id: Id,
    pub name: String,
    pub file_type: FileType,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl FilePreview {
    pub fn new(
        id: Id,
        name: String,
        file_type: FileType,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            file_type,
            created_at,
            modified_at: updated_at,
        }
    }

    // pub fn from_fs(path: &Path, notes_dir: &Path) -> Result<Self> {
    //     let id = Id::id_from_path(path, notes_dir);
    //     let name = path.file_name().unwrap().to_str().unwrap().to_string();
    //     let file_type = match id.get_type() {
    //         Some(file_type) => file_type,
    //         None => return Err(anyhow::anyhow!("Could not get file type")),
    //     };
    //     let created_at = fs::metadata(path)?.created()?;
    //     let updated_at = fs::metadata(path)?.modified()?;
    //
    //     Ok(Self::new(
    //         id,
    //         name,
    //         file_type,
    //         created_at.into(),
    //         updated_at.into(),
    //     ))
    // }
}
