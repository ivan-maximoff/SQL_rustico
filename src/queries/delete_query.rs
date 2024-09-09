use crate::execute::Execute;

use super::comparadores::ExpresionBooleana;

pub struct DeleteQuery {
    pub table: String,
    pub where_clause: Option<ExpresionBooleana>
}

impl DeleteQuery {
    pub fn new(table: &str, where_clause: Option<ExpresionBooleana>) -> Self {
        DeleteQuery {
            table: table.to_string(),
            where_clause,
        }
    }
}

impl Execute for DeleteQuery {
    fn execute(&self, path: &String) -> Result<(), String> {
        Ok(())
    }
}