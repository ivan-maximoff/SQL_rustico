use std::{collections::HashSet, io::BufRead};

use crate::{errores::error::ErrorType, executer::{execute::Execute, manejo_csv::{agregar_linea, agregar_path, crear_archivo, filtrar_columnas, get_reader, imprimir_archivo, listar_columnas, seleccionar_columnas, string_to_columns, where_condition}}};

use super::where_clause::expresion_booleana::ExpresionBooleana;

/// Representa una consulta SQL SELECT con columnas seleccionadas, tabla, cláusula WHERE, orden y columnas para ordenar.
pub struct SelectQuery {
    pub columns_select: HashSet<String>,
    pub table: String,
    pub where_clause: Option<ExpresionBooleana>,
    pub order: String,
    pub by: Vec<String>
}

impl SelectQuery {
     /// Crea una nueva instancia de `SelectQuery`.
    pub fn new(columns_select: Vec<String>, table: String, where_clause: Option<ExpresionBooleana>, order: String, by: Vec<String>) -> Self {
        SelectQuery {
            columns_select: columns_select.into_iter().collect(),
            table,
            where_clause,
            order,
            by
        }
    }
}

impl Execute for SelectQuery {
     /// Ejecuta la consulta SELECT en el archivo especificado, filtrando, seleccionando columnas y ordenando los resultados.
    fn execute(&self, path: &String) -> Result<(), ErrorType> {
        let path_select = agregar_path(path, &self.table);
        let reader = get_reader(&path_select)?;
        let path_aux = agregar_path(&path, &"auxiliar".to_string());
        let _ = crear_archivo(&path_aux)?;

        let lines = reader.lines();
        let (mut lines, columnas) = listar_columnas(&path_aux, lines)?;
        while let Some(line) = lines.next() {
            match line {
                Ok(line) => {
                    let fila = string_to_columns(&line, &columnas)?;
                    if !where_condition(&self.where_clause, &fila)? { 
                        let line = seleccionar_columnas(&fila, &columnas, &self.columns_select);
                        agregar_linea(&path_aux, &line)?;
                    }
                }
                Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
            }
        }
        // Me falto implementar esta parte que no llegue con el tiempo
        // if !self.by.is_empty() { 
        //     ordenar_archivo(&path_aux, &self.order, &self.by)?;
        // }
        let columnas_filtradas: String = filtrar_columnas(&columnas, &self.columns_select);
        imprimir_archivo(&path_aux, columnas_filtradas)?;
        Ok(())
    }
}