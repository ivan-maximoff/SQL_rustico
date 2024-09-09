use std::io::BufRead;

use crate::{errores::error::ErrorType, executer::{execute::Execute, manejo_csv::{agregar_linea, agregar_path, crear_archivo, eliminar_archivo, get_reader, listar_columnas, reemplazar_archivo, string_to_columns, where_condition}}};

use super::where_clause::expresion_booleana::ExpresionBooleana;

/// Representa una consulta SQL DELETE con una tabla y una cláusula WHERE opcional.
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
        let path_delete = agregar_path(path, &self.table);
        let reader = get_reader(&path_delete)?;
        let path_aux = agregar_path(&path, &"auxiliar".to_string());
        let _ = crear_archivo(&path_aux)?;

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