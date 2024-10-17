use crate::errores::error::ErrorType;

/// Trait para ejecutar una consulta SQL. Implementado por diferentes tipos de consultas (`InsertQuery`, `UpdateQuery`, etc.).
pub trait Execute {
    /// Ejecuta la consulta en el archivo especificado y maneja el resultado.
    fn execute(&self, path: &str) -> Result<(), ErrorType>;
}
