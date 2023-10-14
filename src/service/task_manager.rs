use uuid::Uuid;

use crate::model::request::TaskRequest;
use crate::model::task_manager::Task;
use crate::repository::interface::TaskRepositoryInterface;
use crate::repository::task_manager::TaskRepository;
use crate::service::interface::TaskServiceInterface;

use std::error::Error;

pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    pub fn new(repository: TaskRepository) -> Self {
        TaskService {
            repository: repository,
        }
    }
}

impl TaskServiceInterface for TaskService {
    fn insert(
        &mut self,
        task_request: TaskRequest,
        user_id: String,
    ) -> Result<Task, Box<dyn Error>> {
        // validation
        if user_id.is_empty() {
            return Err("user_id cannot be empty".into());
        }

        if task_request.title.is_empty() {
            return Err("title cannot be empty".into());
        }

        if task_request.description.is_empty() {
            return Err("description cannot be empty".into());
        }

        let task = Task {
            id: Uuid::new_v4().to_string(),
            title: task_request.title,
            description: task_request.description,
            completed: task_request.completed,
            owner: user_id,
        };

        let result = self.repository.insert(task)?;
        Ok(result)
    }

    fn find_all(&mut self, user_id: String) -> Result<Vec<Task>, Box<dyn Error>> {
        // validation
        if user_id.is_empty() {
            return Err("user_id cannot be empty".into());
        }

        let tasks = self.repository.find_all(user_id)?;
        Ok(tasks)
    }

    fn find_by_id(
        &mut self,
        task_id: String,
        user_id: String,
    ) -> Result<Option<Task>, Box<dyn Error>> {
        // validation
        if task_id.is_empty() {
            return Err("task_id cannot be empty".into());
        }

        let task = self.repository.find_by_id(task_id, user_id)?;
        Ok(task)
    }

    fn update(
        &mut self,
        task_request: TaskRequest,
        task_id: String,
        user_id: String,
    ) -> Result<Task, Box<dyn Error>> {
        // validation
        if user_id.is_empty() {
            return Err("user_id cannot be empty".into());
        }

        if task_id.is_empty() {
            return Err("task_id cannot be empty".into());
        }

        if task_request.title.is_empty() {
            return Err("title cannot be empty".into());
        }

        if task_request.description.is_empty() {
            return Err("description cannot be empty".into());
        }

        // find task
        let mut task = self.repository.find_by_id(task_id, user_id)?.unwrap();
        task.title = task_request.title;
        task.description = task_request.description;
        task.completed = task_request.completed;

        // update task
        let result = self.repository.update(task)?;
        Ok(result)
    }

    fn delete(&mut self, task_id: String, user_id: String) -> Result<(), Box<dyn Error>> {
        // validation
        if task_id.is_empty() {
            return Err("task_id cannot be empty".into());
        }

        self.repository.delete(task_id, user_id)?;
        Ok(())
    }
}
