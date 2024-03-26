use crate::types::{FilePreview, Id};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}};

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

    pub fn delete_entry(entry_id: Id, notes_dir: &Path) -> Result<()> {
        let entry_path = entry_id.path_from_id(notes_dir);
        let entry_path = entry_path.as_path();

        if !entry_path.exists() {
            return Ok(());
        }

        if entry_path.is_dir() {
            fs::remove_dir_all(entry_path)?;
        }

        if entry_path.is_file() {
            fs::remove_file(entry_path)?;
        }

        Ok(())
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

    pub fn read_notes_dir<F: FnMut(&File)>(notes_dir: &Path, mut f: F) -> Result<Self> {
        Self::read_dir(notes_dir, notes_dir, &mut f)
    }

    pub fn read_dir<F: FnMut(&File)>(
        dir: &Path,
        notes_dir: &Path,
        f: &mut F,
    ) -> Result<Self> {
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
}
