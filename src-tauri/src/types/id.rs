use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use super::FileType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(String);

impl Id {
    pub fn new(path: &Path, notes_dir: &Path) -> Result<Self> {
        let path = match path.strip_prefix(notes_dir) {
            Ok(p) => p,
            Err(_) => return Err(anyhow!("Path is not in notes_dir")),
        };

        let path = path.to_string_lossy().to_string();
        Ok(Self(path))
    }

    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn get_parent(&self, notes_dir: &Path) -> Option<Self> {
        let parts: Vec<&str> = self.0.split('/').collect();
        if parts.len() == 1 {
            None
        } else {
            let parent = parts[..parts.len() - 1].join("/");
            match Self::new(&PathBuf::from(parent), notes_dir) {
                Ok(id) => Some(id),
                Err(_) => None,
            }
        }
    }

    pub fn get_name(&self) -> String {
        let parts: Vec<&str> = self.0.split('/').collect();
        let name = parts.last().unwrap_or(&"");
        let name = name.split('.').next().unwrap_or("");
        name.to_string()
    }

    pub fn get_type(&self) -> Option<FileType> {
        let extension = self.0.split('.').last().unwrap_or("");
        FileType::from_str(extension)
    }

    pub fn create_id(name: &str, parent_id: &Id, file_type: FileType, notes_dir: &Path) -> Self {
        let parent = parent_id.as_str();
        let parent = if parent.is_empty() {
            parent.to_string()
        } else {
            format!("{}/", parent)
        };

        let extension = file_type.to_string();
        let id = format!("{}{}.{}", parent, name, extension);
        Self::new(&PathBuf::from(id), notes_dir).unwrap()
    }

}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

// implement iterator for Id, so we can iterate over the parts of the id
impl IntoIterator for Id {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .split('/')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_id() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let file_path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.md");

        let id = Id::new(&file_path, &notes_dir).unwrap();
        assert_eq!(id.0, "pepa/2021-01-01-123456.md");

        let file_path2 = PathBuf::from("home/user/notes/pepa/2021-01-01-123456.md");
        let id2 = Id::new(&file_path2, &notes_dir).unwrap();
        assert_eq!(id2.0, "pepa/2021-01-01-123456.md");

        let file_path3 = PathBuf::from("user/notes/pepa/2021-01-01-123456.md");
        let id3 = Id::new(&file_path3, &notes_dir).unwrap();
        assert_ne!(id3.0, "pepa/2021-01-01-123456.md");
    }

    #[test]
    fn test_get_parent_directory() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let file_path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.md");
        let parent_path = PathBuf::from("/home/user/notes/pepa");

        let file_id = Id::new(&file_path, &notes_dir).unwrap();
        let parent_id = Id::new(&parent_path, &notes_dir).unwrap();

        let test_parent_id = file_id.get_parent(&notes_dir).unwrap();
        assert_eq!(parent_id, test_parent_id);
    }

    #[test]
    fn test_get_type() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let note_path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.md");
        let image_path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.png");
        let table_path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.csv");
        let todo_path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.todo.csv");

        let note_id = Id::new(&note_path, &notes_dir).unwrap();
        let note_type = note_id.get_type().unwrap();
        assert_eq!(note_type, FileType::Note);

        let image_id = Id::new(&image_path, &notes_dir).unwrap();
        let image_type = image_id.get_type().unwrap();
        assert_eq!(image_type, FileType::Image);

        let table_id = Id::new(&table_path, &notes_dir).unwrap();
        let table_type = table_id.get_type().unwrap();
        assert_eq!(table_type, FileType::Table);

        let todo_id = Id::new(&todo_path, &notes_dir).unwrap();
        let todo_type = todo_id.get_type().unwrap();
        assert_eq!(todo_type, FileType::TodoList);

        let invalid_path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456");
        let invalid_id = Id::new(&invalid_path, &notes_dir).unwrap();
        let invalid_type = invalid_id.get_type();
        assert_eq!(invalid_type, None);
    }

    #[test]
    fn test_iter_id() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let file_path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.md");

        let id = Id::new(&file_path, &notes_dir).unwrap();
        let parts: Vec<String> = id.into_iter().collect();
        assert_eq!(parts, vec!["pepa", "2021-01-01-123456.md"]);
    }
}
