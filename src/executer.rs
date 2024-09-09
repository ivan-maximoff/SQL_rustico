use crate::{execute::Execute, queries::SQLQuery};

pub fn execute_query(path: &String, query: SQLQuery){
    match query.execute(path) {
        Ok(_) => {},
        Err(e) => {println!("{}", e)},
    }
}