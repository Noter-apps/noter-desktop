use std::collections::HashMap;

use crate::types::{FilePreview, FileType, Id};

use super::Entry;

pub enum SortBy {
    Name,
    CreatedAt,
    ModifiedAt,
}

pub enum SortOrder {
    Ascending,
    Descending,
}

pub struct SortOptions {
    pub by: SortBy,
    pub order: SortOrder,
}

pub struct DataAdapter {
    files: HashMap<Id, Entry>,
}

impl DataAdapter {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn sort_files(list: &mut Vec<FilePreview>, sort: SortOptions) {
        let SortOptions { by, order } = sort;
        match by {
            SortBy::Name => {
                list.sort_by(|a, b| {
                    if let SortOrder::Ascending = order {
                        a.name.cmp(&b.name)
                    } else {
                        b.name.cmp(&a.name)
                    }
                });
            }
            SortBy::CreatedAt => {
                list.sort_by(|a, b| {
                    if let SortOrder::Ascending = order {
                        a.created_at.cmp(&b.created_at)
                    } else {
                        b.created_at.cmp(&a.created_at)
                    }
                });
            }
            SortBy::ModifiedAt => {
                list.sort_by(|a, b| {
                    if let SortOrder::Ascending = order {
                        a.modified_at.cmp(&b.modified_at)
                    } else {
                        b.modified_at.cmp(&a.modified_at)
                    }
                });
            }
        }
    }

    pub fn get_all_files(&self, sort: Option<SortOptions>) -> Vec<FilePreview> {
        let mut files: Vec<FilePreview> = self
            .files
            .values()
            .filter_map(|f| match f {
                Entry::File(file) => Some(file),
                _ => None,
            })
            .cloned()
            .collect();
        if let Some(sort) = sort {
            Self::sort_files(&mut files, sort);
        }
        files
    }

    pub fn get_all_notes(&self, sort: Option<SortOptions>) -> Vec<FilePreview> {
        let mut files: Vec<FilePreview> = self
            .files
            .values()
            .filter_map(|f| match f {
                Entry::File(file) => match file.file_type {
                    FileType::Note => Some(file),
                    _ => None,
                },
                _ => None,
            })
            .cloned()
            .filter(|f| f.file_type == FileType::Note)
            .collect();

        if let Some(sort) = sort {
            Self::sort_files(&mut files, sort);
        }
        files
    }

    pub fn get_file(&self, id: &Id) -> Option<&Entry> {
        self.files.get(id)
    }

    pub fn insert_file(&mut self, file: FilePreview) {
        self.files.insert(file.id.clone(), Entry::File(file));
    }

    pub fn delete_file(&mut self, id: &Id) {
        self.files.remove(id);
    }
}
