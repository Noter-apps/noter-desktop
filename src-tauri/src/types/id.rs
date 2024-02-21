use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{File, FileType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id {
    id: String,
    notes_dir: PathBuf,
}

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

        Self {
            id,
            notes_dir: notes_dir.clone(),
        }
    }

    pub fn id_from_path(path: &PathBuf, notes_dir: &PathBuf) -> Result<Self> {
        if !path.starts_with(notes_dir) {
            return Err(anyhow!("Path is not in notes directory"));
        }

        let id = path.strip_prefix(notes_dir)?.to_string_lossy().to_string();
        Ok(Self::new(id, notes_dir))
    }

    pub fn create_child(&self, name: &str, file_type: FileType) -> Self {
        let mut child_id = self.id.clone();
        child_id.push('/');
        child_id.push_str(name);
        child_id.push('.');
        child_id.push_str(&file_type.to_string());
        Id::new(child_id, &self.notes_dir)
    }

    pub fn get_notes_dir(&self) -> &PathBuf {
        &self.notes_dir
    }

    pub fn path_from_id(&self) -> PathBuf {
        self.notes_dir.join(&self.id)
    }

    pub fn get_parent_directory(&self) -> PathBuf {
        let mut path = self.path_from_id();
        path.pop();
        path
    }

    pub fn exists(&self) -> bool {
        self.path_from_id().exists()
    }
}

impl PartialEq<&str> for Id {
    fn eq(&self, other: &&str) -> bool {
        self.id == *other
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.id == *other
    }
}

impl PartialEq<String> for Id {
    fn eq(&self, other: &String) -> bool {
        self.id == *other
    }
}

impl PartialEq<Id> for Id {
    fn eq(&self, other: &Id) -> bool {
        let self_id = self.path_from_id();
        let other_id = other.path_from_id();
        self_id == other_id
    }
}

impl IntoIterator for Id {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.id
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
        self.id.split('/')
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.id)
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
        assert_eq!(id.id, "pepa/2021-01-01-123456.md");

        // from notes dir with trailing slash
        let id = Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir);
        assert_eq!(id.id, "pepa/2021-01-01-123456.md");

        // from absolute path
        let id = Id::new(
            "/home/user/notes/pepa/2021-01-01-123456.md".to_string(),
            &notes_dir,
        );
        assert_eq!(id.id, "pepa/2021-01-01-123456.md");
    }

    #[test]
    fn test_id_from_path() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let path = PathBuf::from("/home/user/notes/pepa/2021-01-01-123456.md");
        let id = Id::id_from_path(&path, &notes_dir).unwrap();
        assert_eq!(id.id, "pepa/2021-01-01-123456.md");
    }

    #[test]
    fn test_path_from_id() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let id = Id::new("2021-01-01-123456.md".to_string(), &notes_dir);
        let path = Id::path_from_id(&id);
        assert_eq!(path, PathBuf::from("/home/user/notes/2021-01-01-123456.md"));
    }

    #[test]
    fn test_get_parent_directory() {
        let notes_dir = PathBuf::from("/home/user/notes");
        let id = Id::new("pepa/2021-01-01-123456.md".to_string(), &notes_dir);
        let parent_dir = id.get_parent_directory();
        assert_eq!(parent_dir, PathBuf::from("/home/user/notes/pepa"));
    }
}
