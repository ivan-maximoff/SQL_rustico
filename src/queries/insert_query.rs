use crate::{dato::Datos, execute::Execute};

pub struct InsertQuery {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<Datos>>,
}
impl InsertQuery {
    pub fn new(table: &str, columns: Vec<String>, values: Vec<Vec<Datos>>) -> Self {
        InsertQuery {
            table: table.to_string(),
            columns,
            values,
        }
    }
}

impl Execute for InsertQuery {
    fn execute(&self, path: &String) -> Result<(), String> {
        Ok(())
    }
}
