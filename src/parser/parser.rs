use std::collections::{HashMap, HashSet};

use crate::{dato::Datos, errores::error::ErrorType, lexer::operador::Operador, queries::{delete_query::DeleteQuery, insert_query::InsertQuery, select_query::SelectQuery, sql_query::SQLQuery, update_query::UpdateQuery, where_clause::{expresion_booleana::ExpresionBooleana, operador_comparacion::OperadorComparacion, valor::Valor}}};

/// Imprime la representación de una lista de operadores en formato legible,
/// indentando los niveles de profundidad para listas anidadas.
/// Creado principalmente validar de forma legible que devuelve de forma correcta
fn printear(rest: &[Operador]){
    fn print_operador(elemento: &Operador, indent: usize) {
        let padding = " ".repeat(indent);
        match elemento {
            Operador::String(s) => println!("{}String: {}", padding, s),
            Operador::Texto(t) => println!("{}Texto: {}", padding, t),
            Operador::Lista(vec) => {
                println!("{}Lista:", padding);
                for item in vec {
                    print_operador(item, indent + 2);
                }
            }
            Operador::Comparador(c) => println!("{}Comparador: {}", padding, c),
        }
    }
    
    for elemento in rest {
        print_operador(elemento, 0);
    }
}

/// Transforma una lista de Operadores en una lista de Strings para la lsita de columnas
/// ["column1", "column2,", "..."] a Vec<String>
fn columns_to_string(lista: &Vec<Operador>) -> Result<Vec<String>, ErrorType> {
    let mut lista_dato: Vec<String> = Vec::new();
    for operador in lista {
        match operador {
            Operador::String(s) | Operador::Texto(s) => lista_dato.push(s.to_string()),
            _ => return Err(ErrorType::InvalidSyntax("Se ingresaron Parentesis o caracteres especiales a las columnas".to_string()))
        }
    }
    Ok(lista_dato)
}

/// Verifica si hay columnas repetidas en un vector de nombres de columnas.
fn columnas_repetidas(columnas: &Vec<String>) -> bool {
    let unique_columns: HashSet<_> = columnas.iter().collect();
    unique_columns.len() != columnas.len()
}

/// Transforma un string a numero
fn string_to_number(s: String) -> Result<Datos, ErrorType> {
    match s.parse::<i64>(){ // Solo se acpetan numeros enteros?
        Ok(num) => Ok(Datos::Integer(num)),
        Err(_) => return Err(ErrorType::InvalidSyntax("Numero invalido en las listas".to_string())),
    }
}

/// Transforma un String o texto en Dato
fn operador_to_dato(operador: &Operador) -> Result<Datos, ErrorType> {
    match operador {
        Operador::String(s) => Ok(string_to_number(s.to_string())?),
        Operador::Texto(s) => Ok(Datos::String(s.to_string())),
        _ => Err(ErrorType::InvalidSyntax("Se esperaba una variable".to_string())),
    }
}

/// Extrae el String mas interno de la lista si no tiene mas de un elemento
fn extraer_interno_lista(operador: &[Operador]) -> Result<Datos, ErrorType> {
    if operador.len() != 1 {return Err(ErrorType::InvalidSyntax("Cantidad de elemenos incorrecta".to_string()))}
    match &operador[0]{
        Operador::String(s) => Ok(string_to_number(s.to_string())?),
        Operador::Texto(s) => Ok(Datos::String(s.to_string())),
        Operador::Lista(operador) => extraer_interno_lista(operador.as_slice()),
        Operador::Comparador(_) => return Err(ErrorType::InvalidSyntax("Comparador inesperado.".to_string()))
    }
}

/// Recibe cualquier operador y lo convierte en un Dato
fn operador_to_single_dato(operador: &Operador) -> Result<Datos, ErrorType> {
    match &operador {
        Operador::String(_) | Operador::Texto(_) => Ok(operador_to_dato(&operador)?),
        Operador::Lista(list) => Ok(extraer_interno_lista(list.as_slice())?),
        Operador::Comparador(_) => Err(ErrorType::InvalidSyntax("Comparador inesperado.".to_string()))
    }
}

/// Extrae el Valor más interno de la lista si no tiene más de un elemento
fn extraer_interno_lista_valor(operador: &[Operador]) -> Result<Valor, ErrorType> {
    if operador.len() != 1 {
        return Err(ErrorType::InvalidSyntax("Cantidad de elementos incorrecta".to_string()));
    }
    match &operador[0] {
        Operador::String(s) => Ok(Valor::String(s.to_string())),
        Operador::Texto(s) => Ok(Valor::Literal(s.to_string())),
        Operador::Lista(operador) => extraer_interno_lista_valor(operador.as_slice()),
        Operador::Comparador(_) => Err(ErrorType::InvalidSyntax("Comparador inesperado.".to_string())),
    }
}

/// Recibe cualquier operador y lo convierte en un Valor -> String o Literal
fn operador_to_single_valor(operador: &Operador) -> Result<Valor, ErrorType> {
    match operador {
        Operador::String(s) => Ok(Valor::String(s.to_string())),
        Operador::Texto(s) => Ok(Valor::Literal(s.to_string())),
        Operador::Lista(list) => extraer_interno_lista_valor(list.as_slice()),
        Operador::Comparador(_) => Err(ErrorType::InvalidSyntax("Comparador inesperado.".to_string())),
    }
}

/// Transforma una lista de Operadores en una lista de Datos para value
/// ["value1", "value2", "..."] a Vec<Datos>
fn operador_to_value(lista: &Vec<Operador>, columnas: &Vec<String>) -> Result<HashMap<String, Datos>, ErrorType> {
    if lista.len() != columnas.len() {
        return Err(ErrorType::InvalidSyntax("El número de columnas y valores no coincide.".to_string()));
    }
    let mut datos: HashMap<String, Datos> = HashMap::new();
    for (i, columna) in columnas.iter().enumerate() {
        let item = operador_to_single_dato(&lista[i])?;
        datos.insert(columna.to_string(), item);
    }
    Ok(datos)
}

/// Para pasar los values a INSERTAR
/// ["value1", "value2", "..."] ["value1", "value2", "..."] ... a Vec<HashMap<String, Datos> -> Vec(Columna -> Valor)
fn operador_to_values(lista: &[Operador], columnas: &Vec<String>) -> Result<Vec<HashMap<String, Datos>>, ErrorType> {
    let mut values: Vec<HashMap<String, Datos>> = Vec::new();
    for operador in lista{
        match operador {
            Operador::Lista(lista) => values.push(operador_to_value(lista, columnas)?),
            _ => return Err(ErrorType::InvalidSyntax("Faltan valores en la consulta INSERT.".to_string()))
        }
    }
    Ok(values)

}

/// Funcion recursiva para guardar cambios de update
fn changes_rec(mut changes: Vec<(String, Datos)>, rest: &[Operador]) -> Result<(Vec<(String, Datos)>, &[Operador]), ErrorType>{
    match rest {
        [] => Ok((changes, rest)),
        [Operador::String(where_str), ..] if where_str == "WHERE" => Ok((changes, rest)),
        [Operador::String(column_str) | Operador::Texto(column_str), Operador::Comparador(igual), value, rest @ ..]
        if igual == "=" => {
            changes.push((column_str.to_string(), operador_to_dato(value)?));
            changes_rec(changes, rest)
        }
        _ => Err(ErrorType::InvalidSyntax("Error de sintaxis en el seteo de columnas = value en UPDATE ".to_string()))
    }
}

/// Comparador a ExpresionComparador
fn string_to_comparacion(comparador: &String) -> Result<OperadorComparacion, ErrorType> {
    match comparador.as_str() {
        "=" => Ok(OperadorComparacion::Igual),
        ">" => Ok(OperadorComparacion::Mayor),
        "<" => Ok(OperadorComparacion::Menor),
        _ => Err(ErrorType::InvalidSyntax("Operador de comparación no válido".to_string())),
    }
}

/// Crea una comparacion simple del formato [izq, =|<|>, der]
fn crear_comparacion(izq: &Operador, comparador: &String, der: &Operador) -> Result<ExpresionBooleana, ErrorType> {
    Ok(ExpresionBooleana::Comparacion {
        izq: operador_to_single_valor(izq)?, 
        operador: string_to_comparacion(comparador)?,
        der: operador_to_single_valor(der)?
    })
}

/// Si a continuacion de una comparacion simple o de una lista el rest esta vacio o no empieza por order terminamos
fn end_where(rest: &[Operador]) -> bool {
    match rest.get(0){
        None => true, // final del slice
        Some(Operador::String(s)) if s == "ORDER" => true,
        // se agregarian otras palabras especiales que frenen el where si las hubiera
        _ => false
    }
}

/// Matchea los operadores con los 5 posibles casos y se llama recursivamente creando un "arbol" de expresiones
fn where_clause_rec(rest: &[Operador]) -> Result<(Option<ExpresionBooleana>, &[Operador]), ErrorType>{
    match rest {
        // [izq, comparador, der] caso de comparacion simple sin que le siga nada
        [izq, Operador::Comparador(comparador), der, rest @ ..]
        if end_where(rest) => {
            let expresion = crear_comparacion(izq, comparador, der)?;
            Ok((Some(expresion), rest))
        },
        // [izq, comparador, der] caso de comparacion simple sin que le siga nada
        [izq, Operador::Comparador(comparador), der, rest @ ..]
        if end_where(rest) => {
            let expresion = crear_comparacion(izq, comparador, der)?;
            Ok((Some(expresion), rest))
        },
        // [lista] lista sin que le siga nada
        [Operador::Lista(lista)] => {
            let (expresion, _) = where_clause_rec(&lista)?;
            Ok((expresion, &[]))
        },
        // [izq, comparador, der, AND | OR, rest] comparacion simple AND | OR y el resto
        [izq, Operador::Comparador(comparador), der, Operador::String(op), rest @ ..] 
        if (op == "AND" || op == "OR") => {
            if rest.is_empty() {return Err(ErrorType::InvalidSyntax("No hay nada despues del AND o OR".to_string()));}
            let operador = if op == "AND" { ExpresionBooleana::And } else { ExpresionBooleana::Or};
            let comparacion = crear_comparacion(izq, comparador, der)?;
        
            let (expresion, rest) = where_clause_rec(rest)?;
            match expresion {
                Some(expr) => Ok((Some(operador(Box::new(comparacion), Box::new(expr))), rest)),
                None => Err(ErrorType::InvalidSyntax("Operador lógico sin expresión válida.".to_string())),
            }
        },
        // [lista, AND | OR, rest] lista AND| OR y el resto
        [Operador::Lista(lista), Operador::String(op), rest @ ..] 
        if op == "AND" || op == "OR" => {
            if rest.is_empty() {return Err(ErrorType::InvalidSyntax("No hay nada despues del AND o OR".to_string()));}
            let operador = if op == "AND" { ExpresionBooleana::And } else { ExpresionBooleana::Or};
        
            let (expresion_izq, _) = where_clause_rec(lista)?;
            let (expresion_der, rest) = where_clause_rec(rest)?;
            
            match (expresion_izq, expresion_der) {
                (Some(expr_izq), Some(expr_der)) => Ok((Some(operador(Box::new(expr_izq), Box::new(expr_der))), rest)),
                _ => Err(ErrorType::InvalidSyntax("Operador lógico sin expresión válida.".to_string())),
            }
        },
        // [NOT, ...]
        [Operador::String(not), rest @ ..] 
        if not == "NOT" => {
            if rest.is_empty() {return Err(ErrorType::InvalidSyntax("No hay nada despues del NOT".to_string()));}
            let (expresion, rest) = where_clause_rec(rest)?;
            match expresion {
                Some(expr) => Ok((Some(ExpresionBooleana::Not(Box::new(expr))), rest)),
                None => Err(ErrorType::InvalidSyntax("No se puede aplicar NOT a una expresión vacía.".to_string())),
            }
        }
        _ =>  Err(ErrorType::InvalidSyntax("Sintaxis invalida en el WHERE".to_string())),
    }
}

/// Se fija si tiene el operador WHERE y si esta devuelve sus valores
fn where_clause(rest: &[Operador]) -> Result<(Option<ExpresionBooleana>, &[Operador]), ErrorType>{
    match rest {
        [Operador::String(where_str), rest @ ..]
        if where_str == "WHERE" => {
            if rest.is_empty() {return Err(ErrorType::InvalidSyntax("Faltan valores despues del WHERE".to_string()))}
            Ok(where_clause_rec(rest)?)
        }
        _ => Ok((None, rest)), // ya que no es un campo obligatorio

    }
}

/// [..., FROM, ...] devuelve las columnas hasta FROM
fn columns_select_rec(rest: &[Operador], mut columns: Vec<String>) -> Result<(Vec<String>, &[Operador]), ErrorType>{
    match rest {
        [Operador::String(from), rest @ ..] if from == "FROM" => Ok((columns, rest)),
        [Operador::String(column) | Operador::Texto(column), rest @ ..] => {
            columns.push(column.to_string());
            columns_select_rec(rest, columns)
        },
        _ => Err(ErrorType::InvalidSyntax("Se esperaba 'FROM' luego de las columnas en SELECT".to_string())),
    }
}

/// Procesa la lista de operadores para extraer las columnas a ordenar.
fn order_by_rec(rest: &[Operador], mut columns: Vec<String>) -> Result<(Vec<String>, &[Operador]), ErrorType> {
    match rest {
        [] => Ok((columns, rest)),
        [Operador::String(column) | Operador::Texto(column)] => {
            columns.push(column.to_string());
            Ok(order_by_rec(rest, columns)?)
        }
        _ => Err(ErrorType::InvalidSyntax("Variables invalidas en SELECT".to_string())),
    }
}

/// Procesa la lista de operadores para extraer las columnas a ordenar.
fn order_by(rest: &[Operador]) -> Result<(String, Vec<String>, &[Operador]), ErrorType> {
    match rest {
        [Operador::String(order), Operador::String(by), rest @ ..]
        if order == "ORDER" && by == "BY" => {
            match rest {
                [] => Err(ErrorType::InvalidSyntax("Faltan valores despues del ORDER BY ".to_string())),
                [Operador::String(column) | Operador::Texto(column), Operador::String(order), rest @ ..]
                if order == "ASC" || order == "DESC" => {
                    let (by, rest) = order_by_rec(rest, vec![column.to_string()])?;
                    Ok((order.to_string(), by, rest))
                }
                [Operador::String(column) | Operador::Texto(column), rest @ ..] => {
                    let (by, rest) = order_by_rec(rest, vec![column.to_string()])?;
                    Ok(("ASC".to_string(), by, rest))
                }
                _ => Err(ErrorType::InvalidSyntax("Variables invalidas en SELECT".to_string())),
            }
        }
        _ => Ok(("ASC".to_string(), Vec::new(), rest)), // ya que no es un campo obligatorio
    }
}



/// PARSERS PARA CADA OPERACION

/// Table, [ "colum1", "column2,", "..."], ["value1", "value2", "..."] ["value1", "value2", "..."] ... a InsertQuery 
fn parser_insert(table: &String, columns: &Vec<Operador>, values: &[Operador])-> Result<InsertQuery, ErrorType>{
    let columns_parsed: Vec<String> = columns_to_string(columns)?;
    if columnas_repetidas(&columns_parsed) {return Err(ErrorType::InvalidSyntax("Columnas repetidas en INSERT".to_string()));}
    let values_parsed: Vec<HashMap<String, Datos>> = operador_to_values(values, &columns_parsed)?;
    Ok(InsertQuery::new(&table.to_string(), columns_parsed, values_parsed))
}

/// Table y [value1, = ,column1, ...] a UpdateQuery
fn parser_update(table: &String, rest: &[Operador])-> Result<UpdateQuery, ErrorType>{
    let (changes, rest) = changes_rec(Vec::new(), rest)?;
    if changes.is_empty() {
        return Err(ErrorType::InvalidSyntax("Faltan columnas y valores en la consulta UPDATE.".to_string()));
    }
    let (where_condition, rest ) = where_clause(rest)?;
    printear(rest);
    if !rest.is_empty() {return Err(ErrorType::InvalidSyntax("Sintaxis invalida en UPDATE".to_string()));}
    Ok(UpdateQuery::new(table.to_string(), changes, where_condition))
}

/// Table y [...] a DeleteQuery
fn parser_delete(table: &String, rest: &[Operador])-> Result<DeleteQuery, ErrorType>{
    let (where_condition, rest ) = where_clause(rest)?;
    if !rest.is_empty() {return Err(ErrorType::InvalidSyntax("Sintaxis invalida en DELETE".to_string()));}
    Ok(DeleteQuery::new(table, where_condition))
}

/// [..., FROM, tabla, WHERE, ..., ORDER, BY] a SelectQuery
fn parser_select(rest: &[Operador])-> Result<SelectQuery, ErrorType>{
    let (columns, rest) = columns_select_rec(rest, Vec::new())?;
    let (table, rest) = match rest {
        [Operador::String(table) | Operador::Texto(table), rest @ ..] => (table, rest),
        _ => return Err(ErrorType::InvalidSyntax("Se esperaba el nombre de la tabla en SELECT".to_string())),
    };
    let (where_condition, rest ) = where_clause(rest)?;
    let (order, by, rest) = order_by(rest)?;
    if !rest.is_empty() {return Err(ErrorType::InvalidSyntax("Sintaxis invalida en SELECT".to_string()));}
    Ok(SelectQuery::new(columns, table.to_string(), where_condition, order, by))
    
}

/// Recibe un string e intenta matchearlo con una Query valida, sino devuelve el error
pub fn parser(query: Vec<Operador>) -> Result<SQLQuery, ErrorType>{
    match query.as_slice(){ // [Operacion, tabla, ...]
        // [INSERT, INTO, tabla, columns, VALUES, values1, values2 ...]
        [Operador::String(insert), Operador::String(into), Operador::String(table), Operador::Lista(columns), Operador::String(values), rest@.. ]
        if insert == "INSERT" && into == "INTO" => {
            if values != "VALUES" {return Err(ErrorType::InvalidSyntax("Falta 'VALUES' en la consulta INSERT.".to_string()))}
            let insert_query: InsertQuery = parser_insert(table, &columns, rest)?;
            Ok(SQLQuery::Insert(insert_query))
        },
        // [UPDATE, tabla, SET, column1, valor1, column1, valor1, ..., WHERE, ...]
        [Operador::String(update), Operador::String(table), Operador::String(set),rest @ ..]
        if update == "UPDATE" => {
            if set != "SET" {return Err(ErrorType::InvalidSyntax("Falta 'SET' en la consulta UPDATE.".to_string()))}
            let update_query: UpdateQuery = parser_update(&table, rest)?;
            Ok(SQLQuery::Update(update_query))
            
        },
        // [DELETE, FROM, tabla, WHERE, ...]
        [Operador::String(delete), Operador::String(from), Operador::String(table) | Operador::Texto(table), rest @ ..]
        if delete == "DELETE" && from == "FROM" => {
            let delete_query: DeleteQuery = parser_delete(&table, rest)?;
            Ok(SQLQuery::Delete(delete_query))
        },
        // [SELECT, ..., FROM, tabla, WHERE, ..., ORDER, BY, ...]
        [Operador::String(select), rest @ ..]
        if select == "SELECT" => {
            let select_query: SelectQuery = parser_select(rest)?;
            Ok(SQLQuery::Select(select_query))
        },
        _ => Err(ErrorType::InvalidSyntax("Query invalida".to_string())),
    }
}