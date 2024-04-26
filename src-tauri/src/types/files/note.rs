use std::path::{Path, PathBuf};

use crate::types::Id;

use super::FileSerializable;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Note {
    body: String,
}

impl Note {
    pub fn new(body: String) -> Self {
        Self { body }
    }

    pub fn get_body(&self) -> &String {
        &self.body
    }

    pub fn find_links(&self, notes_dir: &Path) -> Vec<Id> {
        let mut links = Vec::new();

        self.body.lines().for_each(|line| {
            line.match_indices("[[").for_each(|(start, _)| {
                let mut end = start + 2;
                let mut nested_count = 0;

                while end < line.len() {
                    match line.chars().nth(end) {
                        Some('[') => {
                            if line.chars().nth(end + 1) == Some('[') {
                                nested_count += 1;
                            } else {
                                return;
                            }
                        }
                        Some(']') => {
                            if nested_count > 0 {
                                nested_count -= 1;
                            } else {
                                if line.chars().nth(end + 1) == Some(']') {
                                    break; // Found the closing brackets for valid link
                                }
                                eprintln!("Unbalanced closing bracket: {}", line);
                                return;
                            }
                        }
                        _ => {}
                    };

                    end += 1;
                }

                if end >= line.len() {
                    eprintln!("Unclosed link: {}", line);
                    return;
                }
                let link = &line[start + 2..end];
                let path = PathBuf::from(notes_dir).join(link);
                let id = match Id::new(&path, notes_dir) {
                    Ok(id) => links.push(id),
                    Err(err) => {
                        eprintln!("Error getting id for link {}: {}", link, err);
                    }
                };
            });
        });

        links
    }
}

impl FileSerializable for Note {
    fn custom_deserialize(file_content: &[u8]) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            body: String::from_utf8_lossy(file_content).to_string(),
        }))
    }

    fn custom_serialize(&self) -> Result<Vec<u8>> {
        Ok(self.body.as_bytes().to_vec())
    }
}

// generate tests for the find_links method
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_links() {
        let note = Note::new("[[link1]]\n[[link2]]".to_string());
        let notes_dir = PathBuf::from("/home/user/notes");
        let links = note.find_links(&notes_dir);
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].to_string(), "link1");
        assert_eq!(links[1].to_string(), "link2");
    }

    #[test]
    fn test_find_links_no_links() {
        let note = Note::new("no links".to_string());
        let notes_dir = PathBuf::from("/home/user/notes");
        let links = note.find_links(&notes_dir);
        assert_eq!(links.len(), 0);
    }

    #[test]
    fn test_multiple_links_inline() {
        let note = Note::new("[[link1]] and [[link2]]".to_string());
        let notes_dir = PathBuf::from("/home/user/notes");
        let links = note.find_links(&notes_dir);
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_nested_links() {
        let note = Note::new("[[link1 [[link2]]]]".to_string());
        let notes_dir = PathBuf::from("/home/user/notes");
        let links = note.find_links(&notes_dir);
        assert_eq!(links[0].to_string(), "link2");
    }

    #[test]
    fn test_non_ascii_chars() {
        let note = Note::new("[[Certifikační autorita]]".to_string());
        let notes_dir = PathBuf::from("/home/user/notes");
        let links = note.find_links(&notes_dir);
        assert_eq!(links[0].to_string(), "Certifikační autorita");
    }
}
