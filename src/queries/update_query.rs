use crate::{dato::Datos, execute::Execute};

use super::comparadores::ExpresionBooleana;

pub struct UpdateQuery {
    pub table: String,
    pub changes: Vec<(String, Datos)>,
    pub where_condition: Option<ExpresionBooleana>,
}

impl UpdateQuery{
    pub fn new(table: String, changes: Vec<(String, Datos)>, where_condition: Option<ExpresionBooleana>)-> Self{
        UpdateQuery{
            table,
            changes,
            where_condition
        }
    }
}

impl Execute for UpdateQuery {
    fn execute(&self, path: &String) -> Result<(), String> {
        Ok(())
    }
}