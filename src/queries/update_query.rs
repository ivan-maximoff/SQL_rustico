use std::{collections::HashMap, io::BufRead};

use crate::{dato::Datos, errores::error::ErrorType, executer::{execute::Execute, manejo_csv::{agregar_linea, agregar_path, crear_archivo, eliminar_archivo, get_reader, listar_columnas, modificar_linea, reemplazar_archivo, string_to_columns, where_condition}}};

use super::where_clause::expresion_booleana::ExpresionBooleana;

/// Representa una consulta SQL UPDATE que modifica registros en una tabla.
pub struct UpdateQuery {
    pub table: String,
    pub changes: HashMap<String, Datos>,
    pub where_condition: Option<ExpresionBooleana>,
}

impl UpdateQuery{
     /// Crea una nueva instancia de `UpdateQuery`.
    pub fn new(table: String, changes: Vec<(String, Datos)>, where_condition: Option<ExpresionBooleana>)-> Self{
        let changes: HashMap<String, Datos> = changes.into_iter().collect();
        UpdateQuery{
            table,
            changes,
            where_condition
        }
    }
}


impl Execute for UpdateQuery {
     /// Ejecuta la consulta UPDATE en el archivo especificado, aplicando los cambios a las filas que cumplen la condición WHERE.
    fn execute(&self, path: &String) -> Result<(), ErrorType> {
        let path_update = agregar_path(path, &self.table);
        let reader = get_reader(&path_update)?;
        let path_aux = agregar_path(&path, &"auxiliar".to_string());
        let _ = crear_archivo(&path_aux)?;

        let lines = reader.lines();
        let (mut lines, columnas) = listar_columnas(&path_aux, lines)?;
        while let Some(line) = lines.next() {
            match line {
                Ok(line) => {
                    let fila = string_to_columns(&line, &columnas)?;
                    if !where_condition(&self.where_condition, &fila)? {
                        let line = modificar_linea(&line, &self.changes, &columnas)?;
                        agregar_linea(&path_aux, &line)?;
                    } else{
                        agregar_linea(&path_aux, &line)?;
                    }
                }
                Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
            }
        }

        reemplazar_archivo(&path_aux, &path_update)?;
        eliminar_archivo(&path_aux)?;
        Ok(())
    }
}