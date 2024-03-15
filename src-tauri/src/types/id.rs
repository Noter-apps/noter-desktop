use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::FileType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id(String);

impl Id {
    pub fn new(id: String, notes_dir: &PathBuf) -> Self {
        let id = id
            .trim_start_matches(notes_dir.to_str().unwrap_or(""))
            .to_string();

        // Remove leading slash
        let id = if id.starts_with('/') {
            id.trim_start_matches('/').to_string()
        } else {
            id
        };

        Self(id)
    }

    pub fn id_from_path(path: &PathBuf, notes_dir: &PathBuf) -> Result<Self> {
        if !path.starts_with(notes_dir) {
            return Err(anyhow!("Path is not in notes directory"));
        }

        let id = path.strip_prefix(notes_dir)?.to_string_lossy().to_string();
        Ok(Self::new(id, notes_dir))
    }

    pub fn create_child(&self, name: &str, file_type: FileType, notes_dir: &PathBuf) -> Self {
        let mut child_id = self.0.clone();
        child_id.push('/');
        child_id.push_str(name);
        child_id.push('.');
        child_id.push_str(&file_type.to_string());
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

    pub fn path_from_id(&self, notes_dir: &PathBuf) -> PathBuf {
        notes_dir.join(&self.0)
    }

    pub fn get_parent_directory(&self, notes_dir: &PathBuf) -> PathBuf {
        let mut path = self.path_from_id(notes_dir);
        path.pop();
        path
    }

    pub fn exists(&self, notes_dir: &PathBuf) -> bool {
        self.path_from_id(notes_dir).exists()
    }
}

impl PartialEq<&str> for Id {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for Id {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Id> for Id {
    fn eq(&self, other: &Id) -> bool {
        self.0 == other.0
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
        assert_eq!(id, "pepa/2021-01-01-123456.md");
        assert_eq!(id, "pepa/2021-01-01-123456.md".to_string());
        assert_eq!(
            id,
            Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir)
        );
        assert_eq!(id, id2);

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
    fn test_id_from_path() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.md");
        let id = Id::id_from_path(&path, &notes_dir).unwrap();
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
