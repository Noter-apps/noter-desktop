use crate::types::{FilePreview, Id};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

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

    pub fn delete_entry(entry_id: Id, notes_dir: &PathBuf) -> Result<()> {
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

    pub fn delete_entry(&mut self, entry_id: Id, notes_dir: &PathBuf) -> Result<()> {
        Entry::delete_entry(entry_id.clone(), notes_dir)?;
        self.entries.retain(|entry| entry.get_id() != &entry_id);

        Ok(())
    }

    pub fn get_from_file(id: Id, notes_dir: &PathBuf) -> Result<Self> {
        let mut entries = Vec::new();
        let path = id.path_from_id(notes_dir);
        let metadata = fs::metadata(&path)?;

        for dir_entry in fs::read_dir(&path)? {
            let dir_entry = dir_entry?;
            let dir_entry_path = dir_entry.path();
            let dir_entry_id = Id::id_from_path(&dir_entry_path, &notes_dir)?;

            if dir_entry_path.is_dir() {
                let dir = match Directory::get_from_file(dir_entry_id, notes_dir) {
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
                // const todo
                let file = match File::get_from_file(dir_entry_id.clone(), notes_dir) {
                    Ok(file) => file,
                    Err(e) => {
                        println!("Could not read file from fs; {:?}; {:?}", e, dir_entry_path);
                        continue;
                    }
                };
                let file_preview = FilePreview::new(
                    dir_entry_id,
                    file.name.clone(),
                    file.get_type(),
                    file.created_at,
                    file.modified_at,
                );
                entries.push(Entry::File(file_preview));
            }
        }

        let created_at = metadata.created()?;
        let modified_at = metadata.modified()?;

        Ok(Self::new(
            id,
            path.file_name().unwrap().to_str().unwrap().to_string(),
            entries,
            created_at.into(),
            modified_at.into(),
        ))
    }
}
