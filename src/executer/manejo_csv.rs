use std::{collections::HashMap, fs::{self, File, OpenOptions}, io::{BufRead, BufReader, Lines, Write}};

use crate::{dato::Datos, errores::error::ErrorType, queries::{order_clause::{OrderClause, OrderDirection}, where_clause::{evaluar::Evaluar, expresion_booleana::ExpresionBooleana}}};

/// Abre un archivo en la ruta dada y devuelve un `BufReader` para leer el contenido. Retorna un error si el archivo no se puede abrir.
pub fn get_reader(path: &String) -> Result<BufReader<File>, ErrorType> {
    match File::open(path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(_) => Err(ErrorType::InvalidTable("Error al abrir el archivo".to_string())),
    }
}

/// Copia el archivo de `source_path` a `target_path`, reemplazando el archivo de destino si existe. Retorna un error si la copia falla.
pub fn reemplazar_archivo(source_path: &String, target_path: &String) -> Result<(), ErrorType>{
    match fs::copy(source_path, target_path){
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorType::InvalidTable("Error al reemplazar el archivo".to_string())),
    }
}

/// Elimina el archivo en la ruta dada. Retorna un error si la eliminación falla.
pub fn eliminar_archivo(path: &String) -> Result<(), ErrorType> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorType::InvalidTable("Error al eliminar el archivo".to_string())),
    }
}

/// Genera una ruta de archivo agregando el nombre de la tabla al path dado y añadiendo la extensión `.csv`.
pub fn agregar_path(path: &str, agregado: &String) -> String {
    format!("{}/{}.csv", path, agregado)
}

/// Crea un nuevo archivo en la ruta dada. Retorna un error si la creación del archivo falla.
pub fn crear_archivo(path: &String) -> Result<File, ErrorType>{
    match File::create(path){
        Ok(file) => Ok(file),
        Err(_) => Err(ErrorType::InvalidTable("Error al crear el archivo".to_string()))
    }
}

/// Agrega una línea al final del archivo en la ruta dada. Retorna un error si la apertura o escritura en el archivo falla.
pub fn agregar_linea(path: &String, line: &String) -> Result<(), ErrorType> {
    let Ok(mut file) = OpenOptions::new().append(true).open(path) else {
        return Err(ErrorType::Error(("Error al abrir archivo").to_string()))
    };
    if file.write_all((line.to_string()+"\n").as_bytes()).is_err(){
        return Err(ErrorType::Error(("Error al escribir en archivo").to_string()));
    }
    Ok(())
}

/// Convierte un `HashMap` de datos en una fila de CSV, separando los valores por comas.
pub fn datos_to_row(datos: &HashMap<String, Datos>, columnas: &Vec<String>) -> Result<String, ErrorType> {
    for columna in datos.keys(){
        if datos.get(columna).is_none() {
            return Err(ErrorType::InvalidColumn("Se pasaron columnas inválidas en los datos.".to_string()))
        }
    }
    let mut result = String::new();
    for columna in columnas {
        let value = match datos.get(columna) {
            Some(Datos::Integer(i)) => i.to_string(),
            Some(Datos::String(s)) => s.to_string(),
            None => "".to_string()
        };
        if !result.is_empty() {
            result.push(',');
        }
        result.push_str(&value);
    }
    Ok(result)
}

/// Filtra y devuelve solo las columnas seleccionadas en formato CSV y las posiciones de las mismas.
pub fn filtrar_columnas(columnas_selected: &Vec<String>, columnas: &[String]) -> Result<(String, Vec<usize>), ErrorType> {
    if columnas_selected.len() == 1 && columnas_selected[0] == "*" {
        let columnas_filtradas = columnas.join(",");
        let posiciones: Vec<usize> = (0..columnas.len()).collect();
        return Ok((columnas_filtradas, posiciones));
    }
    let mut columnas_filtradas: String = String::new();
    let mut posiciones: Vec<usize> = Vec::new();
    for columna_selected in columnas_selected{
        let mut pertenece = false;
        for (index, columna) in columnas.iter().enumerate(){
            if columna_selected == columna {
                if !columnas_filtradas.is_empty() {
                    columnas_filtradas.push(',');
                }
                columnas_filtradas.push_str(columna);
                posiciones.push(index);
                pertenece = true;
                break;
            }
        }
        if !pertenece {
            return Err(ErrorType::InvalidColumn("Esa columna no pertenece a la tabla".to_string()));
        }
    }
    Ok((columnas_filtradas, posiciones))
}

/// Lee la primera línea del archivo para obtener los nombres de las columnas y las devuelve junto con el iterador de líneas.
pub fn listar_columnas(path_aux: &String, mut lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> 
Result<(std::io::Lines<std::io::BufReader<std::fs::File>>, Vec<String>), ErrorType> {
    if let Some(line) = lines.next() {
        match line {
            Ok(line) => {
                let column_names: Vec<String> = line
                    .split(',')
                    .map(|column_name| column_name.trim().to_string())
                    .collect();
                agregar_linea(path_aux, &line)?;
                return Ok((lines, column_names));
            },
            Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
        }
    }
    Err(ErrorType::InvalidTable("El archivo está vacío".to_string()))
}

/// Evalúa la condición `where` en una fila y devuelve `true` si la fila cumple con la condición o si no hay condición.
pub fn where_condition(condition: &Option<ExpresionBooleana>, fila: &HashMap<String, Datos> ) -> Result<bool, ErrorType> {
    match condition {
        Some(cond) => Ok(cond.evaluar(fila)?),
        None => Ok(true)
    }
}

/// Convierte una línea de CSV en un `HashMap` de datos, basándose en los nombres de las columnas proporcionados.
pub fn string_to_columns(line: &str, columnas: &[String]) -> Result<HashMap<String, Datos>, ErrorType> {
    let mut result = HashMap::new();
    let values: Vec<&str> = line.split(',').collect();

    if values.len() != columnas.len() {
        return Err(ErrorType::InvalidColumn("Error al escribir una linea".to_string()));
    }

    for (i, columna) in columnas.iter().enumerate() {
        let value = values[i].trim(); 
        let dato = match value.parse::<i32>() {
            Ok(num) => Datos::Integer(num.into()),
            Err(_) => Datos::String(value.to_string()),
        };
        result.insert(columna.to_string(), dato);
    }

    Ok(result)
}

/// Modifica una línea de CSV de acuerdo a los cambios especificados y devuelve la línea modificada.
pub fn modificar_linea(linea: &str, cambios: &HashMap<String, Datos>, columnas: &[String]) -> Result<String, ErrorType>{
    let mut values: Vec<String> = linea.split(',')
        .map(|s| s.trim().to_string()) 
        .collect();

    if values.len() != columnas.len() {
        return Err(ErrorType::InvalidColumn("Error al escribir una linea".to_string()));
    }

    for (i, columna) in columnas.iter().enumerate() {
        if let Some(cambio) = cambios.get(columna) {
            values[i] = match cambio {
                Datos::Integer(num) => num.to_string(),
                Datos::String(s) => s.to_string(),
            };
        }
    }
    Ok(values.join(","))
}

/// Cambia el orden de los valores de las columnas para la query SELECT
fn ordenar_linea(linea: &str, orden: &Vec<usize>) -> Result<String, ErrorType> {
    let values: Vec<String> = linea.split(',')
        .map(|s| s.trim().to_string()) 
        .collect();
    if values.len() < orden.len() {
        return Err(ErrorType::InvalidColumn("Error al escribir una linea".to_string()));
    }
    let mut ordenados: Vec<String> = Vec::new();
    for &index in orden {
        match values.get(index) {
            Some(value) => ordenados.push(value.to_string()),
            None => return Err(ErrorType::InvalidColumn(format!("Índice fuera de rango: {}", index))),
        }
    }
    Ok(ordenados.join(","))
}

/// Imprime el contenido del archivo en la salida estándar, usando las columnas seleccionadas como encabezado y imprimiendo en el orden de las posiciones.
pub fn imprimir_archivo(path: &String, columnas_selected: String, posiciones: Vec<usize>)-> Result<(), ErrorType> {
    let reader = get_reader(path)?;
    let mut lines = reader.lines();
    if lines.next().is_some(){
        println!("{}", columnas_selected)
    };
    for linea in lines {
        match linea {
            Ok(line) =>  println!("{}", ordenar_linea(&line, &posiciones)?),
            Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
        }
    }
    Ok(())
}

/// Compara dos líneas según las cláusulas de orden y las columnas especificadas.
/// Retorna `true` si `linea_1` es posterior que `linea_2` de acuerdo con las cláusulas de orden.
fn comparar_linea(linea_1: &str, linea_2: &str, order_by: &Vec<OrderClause>, columnas: &[String]) -> Result<bool, ErrorType> {
    let linea_1 = string_to_columns(linea_1, columnas)?;
    let linea_2 = string_to_columns(linea_2, columnas)?;
    for clause in order_by {
        let cmp = match clause.direccion {
            OrderDirection::Asc => linea_1.get(&clause.column).cmp(&linea_2.get(&clause.column)),
            OrderDirection::Desc => linea_2.get(&clause.column).cmp(&linea_1.get(&clause.column)),
        };
        if cmp != std::cmp::Ordering::Equal {
            return Ok(cmp == std::cmp::Ordering::Greater);
        }
    }
    Ok(true)
}

/// Lee la primera línea de un iterador de líneas, devolviendo un `Result` con la línea o un error.
fn leer_primera_linea( lines: &mut Lines<BufReader<File>>) -> Result<String, ErrorType> {
    match lines.next() {
        Some(line) => match line {
            Ok(l) => Ok(l),
            Err(_) => Err(ErrorType::InvalidTable("Error al leer una línea".to_string())),
        },
        None => Err(ErrorType::InvalidTable("El archivo está vacío".to_string())),
    }
}

/// Procesa una pasada del algoritmo de bubble sort en el archivo especificado.
/// Reescribe las líneas en el archivo auxiliar y marca si hubo cambios.
fn pasada_bubble_sort_archivo(lines: &mut Lines<BufReader<File>>, path_aux: &String, columnas: &[String], order_by: &Vec<OrderClause>, hay_cambios: &mut bool) -> Result<(), ErrorType> {
    let mut linea_anterior = leer_primera_linea(lines)?;

    for line_actual in lines.by_ref() {
        match line_actual {
            Ok(linea_actual) => {
                if comparar_linea(&linea_actual, &linea_anterior, order_by, columnas)? {
                    agregar_linea(path_aux, &linea_anterior)?;
                    linea_anterior = linea_actual;
                } else {
                    agregar_linea(path_aux, &linea_actual)?;
                    *hay_cambios = true;
                }
        }
            Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
        }
    }
    agregar_linea(path_aux, &linea_anterior)?;
    Ok(())
    
}

/// Ordena el archivo especificado usando el algoritmo de bubble sort basado en las cláusulas de orden.
/// Reemplaza el archivo original con el archivo ordenado.
fn bubble_sort_archivo(path: &str, table: &String, order_by: &Vec<OrderClause>) -> Result<(), ErrorType> {
    if order_by.is_empty() {
        return Ok(());
    }
    let mut hay_cambios = true;
    while hay_cambios {
        hay_cambios = false;

        let (path_original, reader, path_aux) = preparar_archivos(path, table, &"auxiliar_tmp".to_string())?;
        let lines = reader.lines();
        let (mut lines, columnas) = listar_columnas(&path_aux, lines)?;

        pasada_bubble_sort_archivo(&mut lines, &path_aux, &columnas, order_by, &mut hay_cambios)?;

        reemplazar_archivo(&path_aux, &path_original)?;
        eliminar_archivo(&path_aux)?;
    }
    Ok(())
}

/// Ordena un archivo según las cláusulas de orden especificadas.
pub fn ordenar_archivo(path: &str, table: &String, order_by: &Option<Vec<OrderClause>>)->  Result<(), ErrorType> {
    if let Some(order_by) = order_by {
        bubble_sort_archivo(path, table, order_by)?;
    }
    Ok(())
}

pub fn preparar_archivos(path: &str, table: &String, table_auxiliar: &String) -> Result<(String, std::io::BufReader<std::fs::File>, String), ErrorType> {
    let path_table = agregar_path(path, table);
    let reader = get_reader(&path_table)?;
    let path_aux = agregar_path(path, table_auxiliar);
    crear_archivo(&path_aux)?;
    Ok((path_table, reader, path_aux))
}