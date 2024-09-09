pub enum ErrorType{
    InvalidTable(String),
    InvalidColumn(String),
    InvalidSyntax(String),
    Error(String)
}

impl ErrorType {
    pub fn to_string(&self) -> String {
        match self {
            ErrorType::InvalidTable(description) => format!("[INVALID_TABLE]: {}", description),
            ErrorType::InvalidColumn(description) => format!("[INVALID_COLUMN]: {}", description),
            ErrorType::InvalidSyntax(description) => format!("[INVALID_SYNTAX]: {}", description),
            ErrorType::Error(description) => format!("[ERROR]: {}", description),
        }
    }
}