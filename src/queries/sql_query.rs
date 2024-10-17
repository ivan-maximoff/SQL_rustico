use crate::{errores::error::ErrorType, executer::execute::Execute};

use super::{delete_query::DeleteQuery, insert_query::InsertQuery, select_query::SelectQuery, update_query::UpdateQuery};

/// Enum que representa los diferentes tipos de consultas SQL soportadas.
#[derive(Debug, PartialEq)]
pub enum SQLQuery {
    Select(SelectQuery),
    Insert(InsertQuery),
    Update(UpdateQuery),
    Delete(DeleteQuery),
}

impl Execute for SQLQuery {
    /// Ejecuta la consulta SQL según el tipo de consulta.
    fn execute(&self, path: &str) -> Result<(), ErrorType> {
        match self {
            SQLQuery::Select(query) => query.execute(path),
            SQLQuery::Insert(query) => query.execute(path),
            SQLQuery::Update(query) => query.execute(path),
            SQLQuery::Delete(query) => query.execute(path),
        }
    }
}