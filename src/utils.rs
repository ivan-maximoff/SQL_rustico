use crate::{errores::error::ErrorType, executer::execute::Execute, lexer::lexer::lexer, parser::parser::parser};

/// Procesa una consulta SQL: analiza, convierte y ejecuta.
pub fn procesar_consulta(query: &String, path: &String) -> Result<(), ErrorType> {
    let query_lexer = match lexer(query) {
        Ok(query) => query,
        Err(e) => return Err(e),
    };
    let query_parser = match parser(query_lexer) {
        Ok(parsed_query) => parsed_query,
        Err(e) => return Err(e),
    };
    Ok(query_parser.execute(path)?)
}