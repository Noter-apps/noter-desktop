use crate::types::Id;
use anyhow::Result;
use chrono::{DateTime, Utc};
use csv;
use serde::{Deserialize, Serialize};

use super::FileSerializable;

#[derive(Debug, Serialize, Deserialize)]
pub enum Repeat {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
    Task,
    CheckList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    task_id: u32,
    parent_id: Option<u32>,
    task_type: TaskType,
    content: String,
    description: String,
    project: String,
    attachment: Id,
    priority: u8,
    is_completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    start_date: Option<DateTime<Utc>>,
    due_date: DateTime<Utc>,
    reminder: Option<DateTime<Utc>>,
    repeat: Option<Repeat>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TodoList {
    tasks: Vec<Task>,
}


impl FileSerializable for TodoList {
    fn custom_deserialize(file_content: &[u8]) -> Result<Box<Self>> {
        let mut rdr = csv::Reader::from_reader(file_content);
        let mut tasks = Vec::new();

        for result in rdr.deserialize() {
            let task: Task = result?;
            tasks.push(task);
        }

        Ok(Box::new(TodoList { tasks }))
    }

    fn custom_serialize(&self) -> Result<Vec<u8>> {
        let mut wtr = csv::Writer::from_writer(Vec::new());

        for task in &self.tasks {
            wtr.serialize(task)?;
        }

        wtr.flush()?;

        Ok(wtr.into_inner()?)
    }
}
