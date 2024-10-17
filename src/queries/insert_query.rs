use std::{collections::HashMap, io::BufRead};

use crate::{dato::Datos, errores::error::ErrorType, executer::{execute::Execute, manejo_csv::{agregar_linea, datos_to_row, eliminar_archivo, listar_columnas, preparar_archivos, reemplazar_archivo}}};

/// Representa una consulta SQL INSERT con una tabla, columnas y valores.
#[derive(Debug, PartialEq)]
pub struct InsertQuery {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<HashMap<String, Datos>>,
}

impl InsertQuery {
    /// Crea una nueva instancia de `InsertQuery`.
    pub fn new(table: &String, columns: Vec<String>, values: Vec<HashMap<String, Datos>>) -> Self {
        InsertQuery { table: table.to_string(), columns, values }
    }
}

impl Execute for InsertQuery {
    /// Ejecuta la consulta INSERT en el archivo especificado, añadiendo nuevas filas.
    fn execute(&self, path: &str) -> Result<(), ErrorType> {
        let (path_insert, reader, path_aux) = preparar_archivos(path, &self.table, &"auxiliar".to_string())?;
        let lines = reader.lines();
        let (lines, columns) = listar_columnas(&path_aux, lines)?;
        for line in lines {
            match line {
                Ok(line) =>  agregar_linea(&path_aux, &line)?,
                Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
            }
        }

        for value in &self.values{
            let value = datos_to_row(value, &columns)?;
            agregar_linea(&path_aux, &value)?;
        }

        reemplazar_archivo(&path_aux, &path_insert)?;
        eliminar_archivo(&path_aux)?;
        Ok(())
    }
}
