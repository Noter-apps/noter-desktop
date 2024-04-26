use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs::{self, metadata, read_to_string},
    path::{Path, PathBuf},
};

use super::{Directory, Entry, File, FileContent, FileType, Id};

pub struct FileManager {
    notes_dir: PathBuf,
}

impl FileManager {
    pub fn new(notes_dir: PathBuf) -> Self {
        Self { notes_dir }
    }

    pub fn get_path(&self, id: &Id) -> PathBuf {
        let mut path = PathBuf::from(&self.notes_dir);
        path.push(id.as_str());
        path
    }

    pub fn exists(&self, id: &Id) -> bool {
        let path = self.get_path(id);
        path.exists()
    }

    pub fn get_notes_dir(&self) -> &PathBuf {
        &self.notes_dir
    }

    pub fn get_dir_name(&self) -> Result<String> {
        let str = match self.notes_dir.file_name() {
            Some(name) => name.to_str(),
            None => return Err(anyhow!("Notes dir is not a directory")),
        };
        str.map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Notes dir is not a directory"))
    }

    pub fn get_frontmatter(&self, id: &Id) -> Result<HashMap<String, String>> {
        let path = self.get_path(id);

        let content = read_to_string(&path)?;
        let mut lines = content.lines();

        let mut map = HashMap::new();

        if lines.next() != Some("---") {
            return Err(anyhow!("Invalid frontmatter"));
        }

        while let Some(line) = lines.next() {
            if line == "---" {
                break;
            }

            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid frontmatter"));
            }

            map.insert(parts[0].to_string(), parts[1].to_string());
        }

        Ok(map)
    }

    pub fn read(&self, id: &Id) -> Result<File> {
        let path = self.get_path(id);

        if !self.exists(id) {
            return Err(anyhow!("File does not exist"));
        }

        if !path.is_file() {
            return Err(anyhow::anyhow!("Not a File"));
        }

        let metadata = metadata(&path)?;

        let created_at = metadata.created()?;
        let modified_at = metadata.modified()?;

        match path.file_stem() {
            Some(name) => name.to_string_lossy().to_string(),
            None => return Err(anyhow::anyhow!("Could not read file name")),
        };
        let body = match fs::read(&path) {
            Ok(body) => body,
            Err(_) => return Err(anyhow!("File could not be read")),
        };

        let extension = path
            .extension()
            .ok_or_else(|| anyhow::anyhow!("File does not have extension"))
            .map(|ext| ext.to_string_lossy().to_string())?;

        let file_type =
            FileType::from_str(&extension).ok_or_else(|| anyhow!("Invalid extension"))?;

        let file_content = match FileContent::custom_deserialize(&body, file_type) {
            Ok(content) => content,
            Err(_) => return Err(anyhow!("File body has invalid format")),
        };

        Ok(File::new(
            id.clone(),
            file_content,
            created_at.into(),
            modified_at.into(),
        ))
    }

    pub fn save(&self, file: &File) -> Result<()> {
        let path = self.get_path(&file.id);

        let content = match file.content.custom_serialize() {
            Ok(content) => content,
            Err(_) => return Err(anyhow!("Could not serialize file content")),
        };

        match fs::write(&path, content) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("Could not write file")),
        }
    }

    pub fn delete(&self, id: &Id) -> Result<()> {
        let path = self.get_path(id);

        if !self.exists(id) {
            return Err(anyhow!("File does not exist"));
        }

        match fs::remove_file(&path) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("Could not delete file")),
        }
    }

    pub fn copy(&self, id: &Id, new_id: &Id) -> Result<()> {
        let path = self.get_path(id);
        let new_path = self.get_path(new_id);

        if !self.exists(id) {
            return Err(anyhow!("File does not exist"));
        }

        match fs::copy(&path, &new_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow!("Could not copy file")),
        }
    }

    pub fn rename(&self, id: &Id, new_name: &str) -> Result<Id> {
        let path = self.get_path(id);
        let notes_dir = self.get_notes_dir();
        let parent = id
            .get_parent(&notes_dir)
            .unwrap_or_else(|| Id::new(notes_dir, notes_dir).unwrap());
        let new_id = Id::create_id(new_name, &parent, id.get_type().unwrap(), &notes_dir);
        let new_path = self.get_path(&new_id);

        if !self.exists(id) {
            return Err(anyhow!("File does not exist"));
        }

        match fs::rename(&path, &new_path) {
            Ok(_) => Ok(new_id),
            Err(_) => Err(anyhow!("Could not rename file")),
        }
    }

    pub fn read_dir<F: FnMut(&mut File)>(&self, dir: &Path, f: &mut F) -> Result<Directory> {
        let mut entries = Vec::new();
        let metadata = fs::metadata(&self.notes_dir)?;

        for dir_entry in fs::read_dir(dir)? {
            let dir_entry = dir_entry?;
            let dir_entry_path = dir_entry.path();

            if dir_entry_path.is_dir() {
                let dir = match self.read_dir(&dir_entry_path, f) {
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
                let file_id = Id::new(&dir_entry_path, &self.notes_dir)?;

                match self.read(&file_id) {
                    Ok(mut file) => {
                        f(&mut file);
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
        let id = Id::new(&dir, &self.notes_dir)?;

        Ok(Directory::new(
            id,
            dir.file_name().unwrap().to_str().unwrap().to_string(),
            entries,
            created_at.into(),
            modified_at.into(),
        ))
    }

    pub fn read_notes_dir(&self) -> Result<Directory> {
        self.read_dir(&self.notes_dir, &mut |_| {})
    }
}
