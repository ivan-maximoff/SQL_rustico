use std::io::BufRead;

use crate::{errores::error::ErrorType, executer::{execute::Execute, manejo_csv::{agregar_linea, eliminar_archivo, listar_columnas, preparar_archivos, reemplazar_archivo, string_to_columns, where_condition}}};

use super::where_clause::expresion_booleana::ExpresionBooleana;

/// Representa una consulta SQL DELETE con una tabla y una cláusula WHERE opcional.
#[derive(Debug, PartialEq)]
pub struct DeleteQuery {
    pub table: String,
    pub where_clause: Option<ExpresionBooleana>
}

impl DeleteQuery {
     /// Crea una nueva instancia de `DeleteQuery`.
    pub fn new(table: &str, where_clause: Option<ExpresionBooleana>) -> Self {
        DeleteQuery {
            table: table.to_string(),
            where_clause,
        }
    }
}

impl Execute for DeleteQuery {
    /// Ejecuta la consulta DELETE en el archivo especificado, considerando la cláusula WHERE.
    fn execute(&self, path: &String) -> Result<(), ErrorType> {
        let (path_delete, reader, path_aux) = preparar_archivos(path, &self.table, &"auxiliar".to_string())?;
        let lines = reader.lines();
        let (mut lines, columnas) = listar_columnas(&path_aux, lines)?;
        while let Some(line) = lines.next() {
            match line {
                Ok(line) => {
                    let fila = string_to_columns(&line, &columnas)?;
                    if !where_condition(&self.where_clause, &fila)? { agregar_linea(&path_aux, &line)?;}
                }
                Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
            }
        }
        reemplazar_archivo(&path_aux, &path_delete)?;
        eliminar_archivo(&path_aux)?;
        Ok(())
    }
}