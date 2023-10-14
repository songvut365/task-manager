use crate::database::cache::Client;
use crate::model::schema::task::dsl::*;
use crate::model::task_manager::Task;
use crate::repository::interface::TaskRepositoryInterface;
use diesel::{
    delete, insert_into, update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use log::error;
use std::error::Error;

pub struct TaskRepository {
    db_connection: PgConnection,
    redis_client: Client,
}

impl TaskRepository {
    pub fn new(db_connection: PgConnection, redis_client: Client) -> Self {
        TaskRepository {
            db_connection,
            redis_client,
        }
    }
}

impl TaskRepositoryInterface for TaskRepository {
    fn insert(&mut self, new_task: Task) -> Result<Task, Box<dyn Error>> {
        match insert_into(task)
            .values(&new_task)
            .execute(&mut self.db_connection)
        {
            Ok(_) => Ok(new_task),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn find_all(&mut self, user_id: String) -> Result<Vec<Task>, Box<dyn Error>> {
        let key = format!("task::{}", user_id);

        match self.redis_client.get::<Vec<Task>>(key.clone()) {
            Ok(result) => return Ok(result),
            Err(err) => error!("get all task from redis error: {:}", err),
        };

        let db_result = match task
            .filter(owner.eq(user_id.clone()))
            .limit(10)
            .select(Task::as_select())
            .load(&mut self.db_connection)
        {
            Ok(result) => result,
            Err(err) => return Err(Box::new(err)),
        };

        if !db_result.is_empty() {
            match self.redis_client.set(key, &db_result) {
                Ok(_) => return Ok(db_result),
                Err(err) => error!("set all task to redis error: {:}", err),
            }
        }

        Ok(db_result)
    }

    fn find_by_id(
        &mut self,
        task_id: String,
        user_id: String,
    ) -> Result<Option<Task>, Box<dyn Error>> {
        let key = format!("task::{}::{}", user_id, task_id);

        match self.redis_client.get::<Task>(key.clone()) {
            Ok(result) => return Ok(Some(result)),
            Err(err) => error!("get task from redis error: {:}", err),
        }

        let result = task
            .filter(id.eq(task_id))
            .filter(owner.eq(user_id))
            .select(Task::as_select())
            .first(&mut self.db_connection)?;

        self.redis_client
            .set(key, &result)
            .unwrap_or_else(|err| error!("set tasks to redis error: {:}", err));

        Ok(Some(result))
    }

    fn update(&mut self, update_task: Task) -> Result<Task, Box<dyn Error>> {
        let task_id = &update_task.id;
        let user_id = &update_task.owner;

        let key = format!("task::{}::{}", user_id, task_id);

        self.redis_client.delete(key).unwrap_or_else(|err| {
            error!("delete task in redis error: {:}", err);
        });

        match update(task)
            .filter(id.eq(task_id))
            .set(&update_task)
            .execute(&mut self.db_connection)
        {
            Ok(_) => Ok(update_task),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn delete(&mut self, task_id: String, user_id: String) -> Result<(), Box<dyn Error>> {
        let key = format!("task::{}::{}", user_id, task_id);

        self.redis_client.delete(key).unwrap_or_else(|err| {
            error!("delete task in redis error: {:}", err);
        });

        match delete(task)
            .filter(id.eq(task_id))
            .filter(owner.eq(user_id))
            .execute(&mut self.db_connection)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }
}
