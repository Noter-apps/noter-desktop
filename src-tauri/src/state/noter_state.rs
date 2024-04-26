use crate::types::{
    DataAdapter, Directory, Entry, File, FileContent, FileManager, FilePreview, Id, SortOptions,
};
use anyhow::{anyhow, Result};
use chrono::Utc;
use std::{path::PathBuf, sync::Mutex};

pub struct NoterState {
    file_manager: FileManager,
    data_adapter: DataAdapter,
}

impl NoterState {
    pub fn new(notes_dir: PathBuf) -> Result<Self> {
        let file_manager = FileManager::new(notes_dir.clone());
        let data_adapter = DataAdapter::new();

        Ok(Self {
            file_manager,
            data_adapter,
        })
    }

    pub fn get_notes_dir(&self) -> &PathBuf {
        self.file_manager.get_notes_dir()
    }

    pub fn get_all_files(&self, sort: Option<SortOptions>) -> Vec<FilePreview> {
        self.data_adapter.get_all_files(sort)
    }

    pub fn read_notes_dir(&self) -> Result<Directory> {
        self.file_manager.read_notes_dir()
    }

    pub fn create_file(&mut self, file: &File) -> Result<()> {
        self.file_manager.save(file)?;
        self.data_adapter.insert_file(file.get_preview());
        Ok(())
    }

    pub fn delete_file(&mut self, id: &Id) -> Result<()> {
        self.file_manager.delete(&id)?;
        self.data_adapter.delete_file(&id);
        Ok(())
    }

    pub fn get_directory(&self) -> Result<Directory> {
        self.file_manager.read_notes_dir()
    }

    pub fn get_file(&self, id: &Id) -> Result<File> {
        self.file_manager.read(id)
    }

    pub fn get_file_manager(&self) -> &FileManager {
        &self.file_manager
    }

    pub fn update_file(&mut self, id: &Id, content: FileContent) -> Result<File> {
        let file_preview = self
            .data_adapter
            .get_file(id)
            .ok_or_else(|| anyhow!("File not found"))?;

        match file_preview {
            Entry::File(file) => {
                let new_file = File::new(file.id.clone(), content, file.created_at, Utc::now());
                self.file_manager.save(&new_file)?;
                self.data_adapter.insert_file(new_file.get_preview());
                Ok(new_file)
            }
            _ => return Err(anyhow!("Not a file")),
        }
    }
}

pub type HeldState = Mutex<NoterState>;
