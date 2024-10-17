use std::{collections::HashMap, io::BufRead};

use crate::{
    dato::Datos,
    errores::error::ErrorType,
    executer::{
        execute::Execute,
        manejo_csv::{
            agregar_linea, eliminar_archivo, listar_columnas, modificar_linea, preparar_archivos,
            reemplazar_archivo, string_to_columns, where_condition,
        },
    },
};

use super::where_clause::expresion_booleana::ExpresionBooleana;

/// Representa una consulta SQL UPDATE que modifica registros en una tabla.
#[derive(Debug, PartialEq)]
pub struct UpdateQuery {
    pub table: String,
    pub changes: HashMap<String, Datos>,
    pub where_condition: Option<ExpresionBooleana>,
}

impl UpdateQuery {
    /// Crea una nueva instancia de `UpdateQuery`.
    pub fn new(
        table: String,
        changes: HashMap<String, Datos>,
        where_condition: Option<ExpresionBooleana>,
    ) -> Self {
        UpdateQuery {
            table,
            changes,
            where_condition,
        }
    }
}

impl Execute for UpdateQuery {
    /// Ejecuta la consulta UPDATE en el archivo especificado, aplicando los cambios a las filas que cumplen la condición WHERE.
    fn execute(&self, path: &str) -> Result<(), ErrorType> {
        let (path_update, reader, path_aux) =
            preparar_archivos(path, &self.table, &"auxiliar".to_string())?;
        let lines = reader.lines();
        let (lines, columnas) = listar_columnas(&path_aux, lines)?;
        for line in lines {
            match line {
                Ok(mut line) => {
                    let fila = string_to_columns(&line, &columnas)?;
                    if where_condition(&self.where_condition, &fila)? {
                        line = modificar_linea(&line, &self.changes, &columnas)?;
                    }
                    agregar_linea(&path_aux, &line)?;
                }
                Err(_) => {
                    return Err(ErrorType::InvalidTable(
                        "Error al escribir una linea".to_string(),
                    ))
                }
            }
        }
        reemplazar_archivo(&path_aux, &path_update)?;
        eliminar_archivo(&path_aux)?;
        Ok(())
    }
}
