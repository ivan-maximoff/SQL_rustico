use std::fmt::{self, Display, Formatter};

/// Enum que representa los tipos de errores en el sistema.
/// Contiene variantes para errores de tabla, columna, sintaxis y errores generales.
pub enum ErrorType{
    InvalidTable(String),
    InvalidColumn(String),
    InvalidSyntax(String),
    Error(String)
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ErrorType::InvalidTable(description) =>  write!(f, "[INVALID_TABLE]: {}", description),
            ErrorType::InvalidColumn(description) => write!(f, "[INVALID_COLUMN]: {}", description),
            ErrorType::InvalidSyntax(description) => write!(f, "[INVALID_SYNTAX]: {}", description),
            ErrorType::Error(description) => write!(f, "[ERROR]: {}", description),
        }
    }
}
