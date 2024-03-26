use crate::types::{Directory, File, FileContent, FileGraph, FilePreview, FileType, Id};
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};

pub struct NoterState {
    notes_dir: PathBuf,
    files: Vec<FilePreview>,
    dir: Directory,
    graph: FileGraph,
}

impl NoterState {
    pub fn new(notes_dir: PathBuf) -> Result<Self> {
        let mut files = Vec::new();
        let mut graph: HashMap<Id, (bool, Vec<Id>)> = HashMap::new();

        let dir = Directory::read_notes_dir(&notes_dir, |file| {
            if let Some(FileType::Note) = file.id.get_type() {
                let file = File::get_from_file(file.id.clone(), &notes_dir).unwrap();
                files.push(file.get_preview());

                if let FileContent::Note(note) = file.content {
                    let mut links = note.find_links(&notes_dir);
                    links.iter().for_each(|link| {
                        match graph.get_mut(link) {
                            Some((_, idk)) => {
                                idk.push(file.id.clone());
                            }
                            None => {
                                graph.insert(link.clone(), (false, vec![file.id.clone()]));
                            }
                        };
                    });

                    if let Some((_, arr)) = graph.get_mut(&file.id) {
                        links.append(arr);
                    }

                    graph.insert(file.id.clone(), (true, links));
                }
            }
        })?;

        Ok(Self {
            notes_dir,
            files,
            dir,
            graph: FileGraph { nodes: graph },
        })
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
}

pub type HeldState = Mutex<NoterState>;
