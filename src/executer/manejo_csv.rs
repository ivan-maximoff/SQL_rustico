use std::{collections::{HashMap, HashSet}, fs::{self, File, OpenOptions}, io::{BufRead, BufReader, Write}};

use crate::{dato::Datos, errores::error::ErrorType, queries::where_clause::{evaluar::Evaluar, expresion_booleana::ExpresionBooleana}};

/// Abre un archivo en la ruta dada y devuelve un `BufReader` para leer el contenido. Retorna un error si el archivo no se puede abrir.
pub fn get_reader(path: &String) -> Result<BufReader<File>, ErrorType> {
    println!("{}", path);
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
pub fn agregar_path(path: &String, agregado: &String) -> String {
    format!("{}/{}.csv", path, agregado)
}

/// Crea un nuevo archivo en la ruta dada. Retorna un error si la creación del archivo falla.
pub fn crear_archivo(path: &String) -> Result<File, ErrorType>{
    println!("{}", path);
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

/// Filtra y devuelve solo las columnas seleccionadas en formato CSV.
pub fn filtrar_columnas(columnas: &Vec<String>, columns_select: &HashSet<String>) -> String {
    let mut columnas_filtradas: String = String::new();
    for columna in columnas{
        if columns_select.contains(columna) {
            if !columnas_filtradas.is_empty() {
                columnas_filtradas.push(',');
            }
            columnas_filtradas.push_str(columna);
        } 
    }
    columnas_filtradas
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
                agregar_linea(&path_aux, &line)?;
                return Ok((lines, column_names));
            },
            Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
        }
    }
    return Err(ErrorType::InvalidTable("El archivo está vacío".to_string()));
}

/// Evalúa la condición `where` en una fila y devuelve `true` si la fila cumple con la condición o si no hay condición.
pub fn where_condition(condition: &Option<ExpresionBooleana>, fila: &HashMap<String, Datos> ) -> Result<bool, ErrorType> {
    match condition {
        Some(cond) => Ok(cond.evaluar(fila)?),
        None => Ok(true)
    }
}

/// Convierte una línea de CSV en un `HashMap` de datos, basándose en los nombres de las columnas proporcionados.
pub fn string_to_columns(line: &str, columnas: &Vec<String>) -> Result<HashMap<String, Datos>, ErrorType> {
    let mut result = HashMap::new();
    let values: Vec<&str> = line.split(',').collect();

    if values.len() != columnas.len() {
        return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string()));
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
pub fn modificar_linea(linea: &String, cambios: &HashMap<String, Datos>, columnas: &Vec<String>) -> Result<String, ErrorType>{
    let mut values: Vec<String> = linea.split(',')
        .map(|s| s.trim().to_string()) 
        .collect();

    if values.len() != columnas.len() {
        return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string()));
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

/// Selecciona y devuelve los valores de las columnas especificadas en formato de fila CSV.
pub fn seleccionar_columnas(fila: &HashMap<String, Datos>, columnas: &Vec<String>, columns_selected: &HashSet<String>) -> String {
    let mut selected_values: Vec<String> = Vec::new();
    for columna in columnas.iter() {
        if columns_selected.contains(columna) {
            if let Some(dato) = fila.get(columna) {
                let valor = match dato {
                    Datos::Integer(num) => num.to_string(), 
                    Datos::String(s) => s.clone(), 
                };
                selected_values.push(valor);
            }
        }
    }
    selected_values.join(",")
}

/// Imprime el contenido del archivo en la salida estándar, usando las columnas seleccionadas como encabezado, y luego elimina el archivo.
pub fn imprimir_archivo(path: &String, columnas_selected: String)-> Result<(), ErrorType> {
    let reader = get_reader(&path)?;

    let mut lines = reader.lines();
    if let Some(_) = lines.next(){
        println!("{}", columnas_selected)
    };
    while let Some(line) = lines.next() {
        match line {
            Ok(line) =>  println!("{}", line),
            Err(_) => return Err(ErrorType::InvalidTable("Error al escribir una linea".to_string())),
        }
    }

    eliminar_archivo(&path)?;
    Ok(())
}


pub fn ordenar_archivo( path: &String, order: &String, by: &[String])->  Result<(), ErrorType> {
    // let reader = get_reader(path)?;
    // let path_aux = agregar_path(&path, &"auxiliar".to_string());
    // let _ = crear_archivo(&path_aux)?;
    Ok(())
}