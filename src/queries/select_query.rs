use std::io::BufRead;

use crate::{errores::error::ErrorType, executer::{execute::Execute, manejo_csv::{agregar_linea, eliminar_archivo, filtrar_columnas, imprimir_archivo, listar_columnas, ordenar_archivo, preparar_archivos, string_to_columns, where_condition}}};

use super::{order_clause::OrderClause, where_clause::expresion_booleana::ExpresionBooleana};

/// Representa una consulta SQL SELECT con columnas seleccionadas, tabla, cláusula WHERE, orden y columnas para ordenar.
#[derive(Debug, PartialEq)]
pub struct SelectQuery {
    pub columns_select: Vec<String>,
    pub table: String,
    pub where_clause: Option<ExpresionBooleana>,
    pub order_by: Option<Vec<OrderClause>>
}

impl SelectQuery {
    /// Crea una nueva instancia de `SelectQuery`.
    pub fn new(columns_select: Vec<String>, table: String, where_clause: Option<ExpresionBooleana>, order_by: Option<Vec<OrderClause>>) -> Self {
        SelectQuery {
            columns_select,
            table,
            where_clause,
            order_by
        }
    }
}

impl Execute for SelectQuery {
    /// Ejecuta la consulta SELECT en el archivo especificado, filtrando, seleccionando columnas y ordenando los resultados.
    /// Filtra en auxiliar.csv las filas que cumplen el where clause select y luego las ordena e imprime por pantalla.
    fn execute(&self, path: &String) -> Result<(), ErrorType> {
        let (_, reader, path_aux) = preparar_archivos(path, &self.table, &"auxiliar".to_string())?;
        let lines = reader.lines();
        let (mut lines, columnas) = listar_columnas(&path_aux, lines)?;
        while let Some(line) = lines.next() {
            match line {
                Ok(line) => {
                    let fila = string_to_columns(&line, &columnas)?;
                    if where_condition(&self.where_clause, &fila)? { 
                        agregar_linea(&path_aux, &line)?;
                    }
                }
                Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
            }
        }
        ordenar_archivo(&path, &"auxiliar".to_string(), &self.order_by)?;
        let (columnas_filtradas, posiciones) = filtrar_columnas(&self.columns_select, &columnas)?;
        imprimir_archivo(&path_aux, columnas_filtradas, posiciones)?;
        eliminar_archivo(&path_aux)?;
        Ok(())
    }
}