use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::FileType;

/// Id represents unique identificator for a file in notes_dir.
/// It is basically filepath relative from the notes_dir
///
/// Id does not guarantee that a file on this path exists, that needs to be check by the caller
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Id(String);

impl Id {
    pub fn new(id: impl Into<String>, notes_dir: &Path) -> Self {
        let id = id.into();
        let notes_dir = notes_dir.to_string_lossy().to_string();

        let notes_dir = if notes_dir.ends_with('/') {
            notes_dir
        } else {
            format!("{}/", notes_dir)
        };

        let id = if id.starts_with(&notes_dir) {
            &id[notes_dir.len()..]
        } else {
            &id
        };

        Self(id.to_owned())
    }

    pub fn id_from_path(path: &Path, notes_dir: &Path) -> Self {
        let id = path.to_string_lossy().to_string();
        Self::new(id, notes_dir)
    }

    pub fn create_child(&self, name: &str, file_type: FileType, notes_dir: &Path) -> Self {
        let child_id = format!("{}/{}.{}", self.0, name, file_type.to_string());
        Id::new(child_id, notes_dir)
    }

    pub fn get_type(&self) -> Option<FileType> {
        let parts: Vec<&str> = self.0.split('.').collect();
        let file_type = parts.last().unwrap_or(&"");
        match file_type.as_ref() {
            "md" => Some(FileType::Note),
            "todo.csv" => Some(FileType::TodoList),
            "png" => Some(FileType::Image),
            "csv" => Some(FileType::Table),
            _ => None,
        }
    }

    pub fn path_from_id(&self, notes_dir: &Path) -> PathBuf {
        notes_dir.join(&self.0)
    }

    pub fn get_parent_directory(&self, notes_dir: &Path) -> PathBuf {
        let mut path = self.path_from_id(notes_dir);
        path.pop();
        path
    }

    pub fn exists(&self, notes_dir: &Path) -> bool {
        self.path_from_id(notes_dir).exists()
    }
}

impl IntoIterator for Id {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .split('/')
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<'a> IntoIterator for &'a Id {
    type Item = &'a str;
    type IntoIter = std::str::Split<'a, char>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.split('/')
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let id = Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir);
        let id2 = Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir);
        assert_eq!(id, id2);
        assert_eq!(
            id,
            Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir)
        );
        assert_eq!(id.to_string(), id2.to_string());
        assert!(id == id2);
    }

    #[test]
    fn test_new_id() {
        let notes_dir = PathBuf::from("/home/user/notes");
        // from notes dir
        let id = Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir);
        assert_eq!(id.0, "pepa/2021-01-01-123456.md");

        // from notes dir with trailing slash
        let id = Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir);
        assert_eq!(id.0, "pepa/2021-01-01-123456.md");

        // from absolute path
        let id = Id::new(
            "/home/user/notes/pepa/2021-01-01-123456.md".to_string(),
            &notes_dir,
        );
        assert_eq!(id.0, "pepa/2021-01-01-123456.md");
    }

    #[test]
    fn test_path_from_id() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let id = Id::new("2021-01-01-123456.md".to_string(), &notes_dir);
        let path = Id::path_from_id(&id, &notes_dir);
        assert_eq!(path, PathBuf::from("/home/user/notes/2021-01-01-123456.md"));
    }

    #[test]
    fn test_get_parent_directory() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let id = Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir);
        let parent_dir = id.get_parent_directory(&notes_dir);
        assert_eq!(parent_dir, PathBuf::from("/home/user/notes/pepa"));
    }
}
