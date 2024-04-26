use crate::types::{FilePreview, Id};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{ fs, path::Path};

use super::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Entry {
    File(FilePreview),
    Directory(Directory),
}

impl Entry {
    pub fn get_id(&self) -> &Id {
        match self {
            Entry::File(file) => &file.id,
            Entry::Directory(dir) => &dir.id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Directory {
    pub id: Id,
    pub name: String,
    pub entries: Vec<Entry>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Directory {
    pub fn new(
        id: Id,
        name: String,
        entries: Vec<Entry>,
        created_at: DateTime<Utc>,
        modified_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            entries,
            created_at,
            modified_at,
        }
    }
}

/* 
     pub fn read_notes_dir<F: FnMut(&File)>(notes_dir: &Path, mut f: F) -> Result<Self> {
        Self::read_dir(notes_dir, notes_dir, &mut f)
    }

    pub fn read_dir<F: FnMut(&File)>(dir: &Path, notes_dir: &Path, f: &mut F) -> Result<Self> {
        let mut entries = Vec::new();
        let metadata = fs::metadata(notes_dir)?;

        for dir_entry in fs::read_dir(dir)? {
            let dir_entry = dir_entry?;
            let dir_entry_path = dir_entry.path();

            if dir_entry_path.is_dir() {
                let dir = match Self::read_dir(&dir_entry_path, notes_dir, f) {
                    Ok(dir) => dir,
                    Err(e) => {
                        println!(
                            "Could not read directory from fs; {:?}; {:?}",
                            e, dir_entry_path
                        );
                        continue;
                    }
                };
                entries.push(Entry::Directory(dir));
            } else if dir_entry_path.is_file() {
                let file_id = Id::id_from_path(&dir_entry_path, notes_dir);
                match File::get_from_file(file_id, notes_dir) {
                    Ok(file) => {
                        f(&file);
                        entries.push(Entry::File(file.get_preview()));
                    }
                    Err(e) => {
                        println!("Could not read file from fs; {:?}; {:?}", e, dir_entry_path);
                        continue;
                    }
                }
            }
        }

        let created_at = metadata.created()?;
        let modified_at = metadata.modified()?;
        let id = Id::id_from_path(dir, notes_dir);

        Ok(Directory::new(
            id,
            dir.file_name().unwrap().to_str().unwrap().to_string(),
            entries,
            created_at.into(),
            modified_at.into(),
        ))
    }

    pub fn delete_entry(&mut self, entry_id: Id, notes_dir: &Path) -> Result<()> {
        Entry::delete_entry(entry_id.clone(), notes_dir)?;
        self.entries.retain(|entry| entry.get_id() != &entry_id);

        Ok(())
    }

    fn add_entry(&mut self, entry: Entry, notes_dir: &Path) {
        let entry_id = entry.get_id();
        let entry_parent_path = entry_id.get_parent_directory(&notes_dir);
        let dir_path = self.id.path_from_id(&notes_dir);

        if dir_path == entry_parent_path {
            self.entries.push(entry);
            return;
        }

        for dir_entry in self.entries.iter_mut() {
            if let Entry::Directory(dir) = dir_entry {
                dir.add_entry(entry.clone(), notes_dir);
            }
        }
    }
}

// create unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FileContent, FileType};
    use std::fs::File;

    #[test]
    fn test_directory_read_notes_dir() {
        let dir = tempdir().unwrap();
        let notes_dir = dir.path().join("notes");
        fs::create_dir(&notes_dir).unwrap();

        let note = notes_dir.join("note.md");
        File::create(&note).unwrap();

        let dir = Directory::read_notes_dir(&notes_dir).unwrap();
        assert_eq!(dir.entries.len(), 1);
    }

    #[test]
    fn test_directory_delete_entry() {
        let dir = tempdir().unwrap();
        let notes_dir = dir.path().join("notes");
        fs::create_dir(&notes_dir).unwrap();

        let note = notes_dir.join("note.md");
        File::create(&note).unwrap();

        let mut dir = Directory::read_notes_dir(&notes_dir).unwrap();
        let note_id = Id::id_from_path(&note, &notes_dir);
        dir.delete_entry(note_id.clone(), &notes_dir).unwrap();
        assert_eq!(dir.entries.len(), 0);
    }

    #[test]
    fn test_directory_add_entry() {
        let dir = tempdir().unwrap();
        let notes_dir = dir.path().join("notes");
        fs::create_dir(&notes_dir).unwrap();

        let mut dir = Directory::read_notes_dir(&notes_dir).unwrap();
        let note = notes_dir.join("note.md");
        File::create(&note).unwrap();
        let note_id = Id::id_from_path(&note, &notes_dir);
        let file = File::get_from_file(note_id.clone(), &notes_dir).unwrap();
        let entry = Entry::File(file.get_preview());
        dir.add_entry(entry, &notes_dir);

        assert_eq!(dir.entries.len(), 1);
    }
}
*/

