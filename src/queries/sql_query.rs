use crate::execute::Execute;

use super::{DeleteQuery, InsertQuery, SelectQuery, UpdateQuery};

pub enum SQLQuery {
    Select(SelectQuery),
    Insert(InsertQuery),
    Update(UpdateQuery),
    Delete(DeleteQuery),
}

impl Execute for SQLQuery {
    fn execute(&self, path: &String) -> Result<(), String> {
        match self {
            SQLQuery::Select(query) => query.execute(path),
            SQLQuery::Insert(query) => query.execute(path),
            SQLQuery::Update(query) => query.execute(path),
            SQLQuery::Delete(query) => query.execute(path),
        }
    }
}