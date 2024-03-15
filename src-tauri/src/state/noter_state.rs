use anyhow::Result;
use std::{fs, path::PathBuf, sync::Mutex};

use crate::types::{Directory, Entry, FilePreview, Id};

pub struct NoterState {
    notes_dir: PathBuf,
    files: Vec<FilePreview>,
    dir: Directory,
}

impl NoterState {
    pub fn new(notes_dir: PathBuf) -> Self {
        Self::from_fs(notes_dir).unwrap()
    }

    pub fn get_notes_dir(&self) -> &PathBuf {
        &self.notes_dir
    }

    pub fn get_files(&self) -> &Vec<FilePreview> {
        &self.files
    }

    pub fn get_directory(&self) -> &Directory {
        &self.dir
    }

    fn read_dir_rec(
        path: &PathBuf,
        notes_dir: &PathBuf,
        files: &mut Vec<FilePreview>,
    ) -> Result<Directory> {
        let mut entries = Vec::new();
        let metadata = fs::metadata(&path)?;

        for dir_entry in fs::read_dir(&path)? {
            let dir_entry = dir_entry?;
            let dir_entry_path = dir_entry.path();

            if dir_entry_path.is_dir() {
                let dir = match Self::read_dir_rec(&dir_entry_path, notes_dir, files) {
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
                let file_preview = match FilePreview::from_fs(&dir_entry_path, notes_dir) {
                    Ok(file_preview) => file_preview,
                    Err(e) => {
                        println!("Could not read file from fs; {:?}; {:?}", e, dir_entry_path);
                        continue;
                    }
                };
                files.push(file_preview.clone());
                entries.push(Entry::File(file_preview));
            }
        }

        let created_at = metadata.created()?;
        let modified_at = metadata.modified()?;
        let dir_id = Id::id_from_path(path, notes_dir)?;

        Ok(Directory::new(
            dir_id,
            path.file_name().unwrap().to_str().unwrap().to_string(),
            entries,
            created_at.into(),
            modified_at.into(),
        ))
    }

    fn from_fs(notes_dir: PathBuf) -> Result<Self> {
        let mut files = Vec::new();

        let dir = Self::read_dir_rec(&notes_dir, &notes_dir, &mut files)?;

        Ok(Self {
            notes_dir,
            files,
            dir,
        })
    }
}

pub type HeldState = Mutex<NoterState>;
