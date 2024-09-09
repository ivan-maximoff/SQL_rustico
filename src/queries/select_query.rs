use crate::execute::Execute;

use super::comparadores::ExpresionBooleana;

pub struct SelectQuery {
    pub columns: Vec<String>,
    pub table: String,
    pub where_clause: Option<ExpresionBooleana>,
    pub order: String,
    pub by: Vec<String>
}

impl SelectQuery {
    pub fn new(columns: Vec<String>, table: String, where_clause: Option<ExpresionBooleana>, order: String, by: Vec<String>) -> Self {
        SelectQuery {
            columns,
            table,
            where_clause,
            order,
            by
        }
    }
}

impl Execute for SelectQuery {
    fn execute(&self, path: &String) -> Result<(), String> {
        Ok(())
    }
}