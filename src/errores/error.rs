/// Enum que representa los tipos de errores en el sistema.
/// Contiene variantes para errores de tabla, columna, sintaxis y errores generales.
pub enum ErrorType{
    InvalidTable(String),
    InvalidColumn(String),
    InvalidSyntax(String),
    Error(String)
}

impl ErrorType {
    /// Convierte el tipo de error en una cadena de texto con formato específico.
    pub fn to_string(&self) -> String {
        match self {
            ErrorType::InvalidTable(description) => format!("[INVALID_TABLE]: {}", description),
            ErrorType::InvalidColumn(description) => format!("[INVALID_COLUMN]: {}", description),
            ErrorType::InvalidSyntax(description) => format!("[INVALID_SYNTAX]: {}", description),
            ErrorType::Error(description) => format!("[ERROR]: {}", description),
        }
    }
}