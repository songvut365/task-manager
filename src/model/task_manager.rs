use crate::model::schema;
use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(
    Serialize, Deserialize, Debug, Queryable, Selectable, AsChangeset, Insertable, Default,
)]
#[diesel(table_name = schema::task)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub owner: String,
}
