use std::{collections::HashMap, io::BufRead};

use crate::{dato::Datos, errores::error::ErrorType, executer::{execute::Execute, manejo_csv::{agregar_linea, agregar_path, crear_archivo, datos_to_row, eliminar_archivo, get_reader, listar_columnas, reemplazar_archivo}}};

/// Representa una consulta SQL INSERT con una tabla, columnas y valores.
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
    fn execute(&self, path: &String) -> Result<(), ErrorType> {
        let path_insert = agregar_path(path, &self.table);
        let reader = get_reader(&path_insert)?;
        let path_aux = agregar_path(&path, &"auxiliar".to_string());
        let _ = crear_archivo(&path_aux)?;

        let lines = reader.lines();
        let (mut lines, columns) = listar_columnas(&path_aux, lines)?;

        while let Some(line) = lines.next() {
            match line {
                Ok(line) =>  agregar_linea(&path_aux, &line)?,
                Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
            }
        }

        for value in &self.values{
            let value = datos_to_row(&value, &columns)?;
            agregar_linea(&path_aux, &value)?;
        }

        reemplazar_archivo(&path_aux, &path_insert)?;
        eliminar_archivo(&path_aux)?;
        Ok(())
    }
}
