use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::task_manager::Task;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResponse {
    pub code: String,
    pub description: String,
    pub data: Option<TaskResponseData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TaskResponseData {
    Task(Task),
    Tasks(Vec<Task>),
}

pub fn create_task_response(
    code: &str,
    description: &str,
    data: Option<TaskResponseData>,
) -> TaskResponse {
    TaskResponse {
        code: String::from(code),
        description: String::from(description),
        data: data,
    }
}
