use crate::model::request::TaskRequest;
use crate::model::task_manager::Task;

use std::error::Error;

pub trait TaskServiceInterface {
    fn insert(
        &mut self,
        task_request: TaskRequest,
        user_id: String,
    ) -> Result<Task, Box<dyn Error>>;
    fn find_all(&mut self, user_id: String) -> Result<Vec<Task>, Box<dyn Error>>;
    fn find_by_id(
        &mut self,
        task_id: String,
        user_id: String,
    ) -> Result<Option<Task>, Box<dyn Error>>;
    fn update(
        &mut self,
        task_request: TaskRequest,
        task_id: String,
        user_id: String,
    ) -> Result<Task, Box<dyn Error>>;
    fn delete(&mut self, task_id: String, user_id: String) -> Result<(), Box<dyn Error>>;
}
