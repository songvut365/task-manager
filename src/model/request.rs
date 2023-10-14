use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskRequest {
    pub title: String,
    pub description: String,
    pub completed: bool,
}
