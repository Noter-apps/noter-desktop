use std::fs;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::Id;

use super::{Image, Note, Table, TodoList};

pub const IMAGE_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

pub trait FileSerializable {
    fn custom_serialize(&self) -> Result<Vec<u8>>;
    fn custom_deserialize(file_content: &[u8]) -> Result<Box<Self>>;
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum FileContent {
    Note(Box<Note>),
    TodoList(Box<TodoList>),
    Image(Box<Image>),
    Table(Box<Table>),
}

impl FileContent {
    pub fn default(file_type: FileType) -> Self {
        match file_type {
            FileType::Note => Self::Note(Box::new(Note::default())),
            FileType::TodoList => Self::TodoList(Box::new(TodoList::default())),
            FileType::Image => Self::Image(Box::new(Image::default())),
            FileType::Table => Self::Table(Box::new(Table::default())),
        }
    }
}

impl FileContent {
    pub fn custom_serialize(&self) -> Result<Vec<u8>> {
        match self {
            Self::Note(note) => note.custom_serialize(),
            Self::TodoList(todo_list) => todo_list.custom_serialize(),
            Self::Image(image) => image.custom_serialize(),
            Self::Table(table) => table.custom_serialize(),
        }
    }

    pub fn custom_deserialize(file_content: &[u8], file_type: FileType) -> Result<Self> {
        match file_type {
            FileType::Note => Ok(Self::Note(Note::custom_deserialize(file_content)?)),
            FileType::TodoList => Ok(Self::TodoList(TodoList::custom_deserialize(file_content)?)),
            FileType::Image => Ok(Self::Image(Image::custom_deserialize(file_content)?)),
            FileType::Table => Ok(Self::Table(Table::custom_deserialize(file_content)?)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: Id,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub content: FileContent,
}

impl File {
    pub fn new(id: Id, name: String, content: FileContent) -> Result<Self> {
        let now = Utc::now();

        let new_file = Self {
            id,
            name,
            created_at: now,
            modified_at: now,
            content,
        };

        new_file.save_to_file()?;
        Ok(new_file)
    }

    pub fn get_from_file(id: Id) -> Result<Self> {
        let path = id.path_from_id();

        if !path.exists() {
            return Err(anyhow::anyhow!("File does not exist"));
        }

        if !path.is_file() {
            return Err(anyhow::anyhow!("Not a File"));
        }

        if !path.starts_with(id.get_notes_dir()) {
            return Err(anyhow::anyhow!("File is not in notes directory"));
        }

        let metadata = fs::metadata(&path)?;

        let created_at = metadata.created()?;
        let modified_at = metadata.modified()?;

        let name = match path.file_stem() {
            Some(name) => name.to_string_lossy().to_string(),
            None => return Err(anyhow::anyhow!("Could not get file name")),
        };
        let extension = match path.extension() {
            Some(extension) => extension.to_string_lossy().to_string(),
            None => return Err(anyhow::anyhow!("Could not get file extension")),
        };

        let content = fs::read(&path)?;

        let file_type = match extension.as_str() {
            "md" => FileType::Note,
            "csv" if name.contains(".todo") => FileType::TodoList,
            "csv" => FileType::Table,
            _ if IMAGE_EXTENSIONS.contains(&extension.as_str()) => FileType::Image,
            _ => return Err(anyhow::anyhow!("Unknown file extension")),
        };

        let content = FileContent::custom_deserialize(&content, file_type)?;

        Ok(Self {
            id,
            name,
            created_at: created_at.into(),
            modified_at: modified_at.into(),
            content,
        })
    }

    pub fn delete(id: Id) -> Result<()> {
        fs::remove_file(id.path_from_id())?;

        Ok(())
    }

    pub fn save_to_file(&self) -> Result<()> {
        let path = self.id.path_from_id();

        fs::write(&path, self.content.custom_serialize()?)?;

        Ok(())
    }

    pub fn put(&mut self, content: FileContent) -> Result<()> {
        self.content = content;
        self.modified_at = Utc::now();


        self.save_to_file()
    }

    pub fn rename(&mut self, mut new_name: String) -> Result<()> {
        let old_path = self.id.path_from_id();

        let notes_dir = self.id.get_notes_dir();
        new_name.push('.');
        new_name.push_str(&self.get_type().to_string());
        let new_id = Id::new(new_name.clone(), notes_dir);

        let new_path = new_id.path_from_id();

        fs::rename(old_path, new_path)?;

        self.id = new_id;
        self.name = new_name;

        Ok(())
    }

    pub fn get_type(&self) -> FileType {
        match &self.content {
            FileContent::Note(_) => FileType::Note,
            FileContent::TodoList(_) => FileType::TodoList,
            FileContent::Image(_) => FileType::Image,
            FileContent::Table(_) => FileType::Table,
        }
    }
}
