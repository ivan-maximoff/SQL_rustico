use std::collections::{HashMap, HashSet};

use crate::{dato::Datos, errores::error::ErrorType, lexer::operador::Operador, queries::{delete_query::DeleteQuery, insert_query::InsertQuery, order_clause::{OrderClause, OrderDirection}, select_query::SelectQuery, sql_query::SQLQuery, update_query::UpdateQuery, where_clause::expresion_booleana::ExpresionBooleana}, utils::{operador_to_dato, operador_to_single_dato, operador_to_single_valor, string_to_comparacion, string_to_direccion}};

/// Transforma una lista de Operadores en una lista de Strings para la lista de columnas
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
fn changes_rec(mut changes: HashMap<String, Datos>, rest: &[Operador]) -> Result<(HashMap<String, Datos>, &[Operador]), ErrorType>{
    match rest {
        [] => Ok((changes, rest)),
        [Operador::String(where_str), ..] if where_str == "WHERE" => Ok((changes, rest)),
        [Operador::String(column_str) | Operador::Texto(column_str), Operador::Comparador(igual), value, rest @ ..]
        if igual == "=" => {
            changes.insert(column_str.to_string(), operador_to_dato(value)?);
            changes_rec(changes, rest)
        }
        _ => Err(ErrorType::InvalidSyntax("Error de sintaxis en el seteo de columnas = value en UPDATE ".to_string()))
    }
}

/// Crea una comparacion simple del formato [izq, =|<|>|>=|<=, der]
fn crear_comparacion(izq: &Operador, comparador: &String, der: &Operador) -> Result<ExpresionBooleana, ErrorType> {
    Ok(ExpresionBooleana::Comparacion {
        izq: operador_to_single_valor(izq)?, 
        operador: string_to_comparacion(comparador)?,
        der: operador_to_single_valor(der)?
    })
}

/// Determina si se debe finalizar el procesamiento del WHERE
fn end_where(rest: &[Operador]) -> bool {
    match rest.get(0) {
        None => true, // Final del slice
        Some(Operador::String(s)) if s == "ORDER" => true,
        _ => false,
    }
}

/// Maneja operadores lógicos AND y OR
fn procesar_operador_logico(op: String, comparacion: ExpresionBooleana, rest: &[Operador] ) -> Result<(Option<ExpresionBooleana>, &[Operador]), ErrorType> {
    let operador = if op == "AND" { ExpresionBooleana::And } else { ExpresionBooleana::Or };
    if rest.is_empty() {return Err(ErrorType::InvalidSyntax(format!("No hay nada después del {}.", op)));}
    let (expresion, rest) = where_clause_rec(rest)?;
    match expresion {
        Some(expr) => Ok((Some(operador(Box::new(comparacion), Box::new(expr))), rest)),
        None => Err(ErrorType::InvalidSyntax(format!("Operador lógico {} sin expresión válida.", op))),
    }
}

/// Maneja la clausula NOT
fn procesar_not(rest: &[Operador]) -> Result<(Option<ExpresionBooleana>, &[Operador]), ErrorType> {
    if rest.is_empty() {return Err(ErrorType::InvalidSyntax("No hay nada después del NOT.".to_string()));}
    let (expresion, rest) = where_clause_rec(rest)?;
    match expresion {
        Some(expr) => Ok((Some(ExpresionBooleana::Not(Box::new(expr))), rest)),
        None => Err(ErrorType::InvalidSyntax("No se puede aplicar NOT a una expresión vacía.".to_string())),
    }
}

/// Matchea los operadores con los 5 posibles casos y se llama recursivamente creando un "arbol" de expresiones
fn where_clause_rec(rest: &[Operador]) -> Result<(Option<ExpresionBooleana>, &[Operador]), ErrorType> {
    match rest {
        // [izq, comparador, der] caso de comparacion simple sin que le siga nada
        [izq, Operador::Comparador(comparador), der, rest @ ..] if end_where(rest) => {
            let expresion = crear_comparacion(izq, comparador, der)?;
            Ok((Some(expresion), rest))
        },
        // [izq, comparador, der, AND | OR, rest] comparacion simple AND | OR y el resto
        [izq, Operador::Comparador(comparador), der, Operador::String(op), rest @ ..] 
        if op == "AND" || op == "OR" => {
            let comparacion = crear_comparacion(izq, comparador, der)?;
            procesar_operador_logico(op.to_string(), comparacion, rest)
        },
        // [lista] lista sin que le siga nada
        [Operador::Lista(lista), rest @..] if end_where(rest) => {
            let (expresion, _) = where_clause_rec(&lista)?;
            Ok((expresion, rest))
        },
        // [lista, AND | OR, rest] lista AND | OR y el resto
        [Operador::Lista(lista), Operador::String(op), rest @ ..] if op == "AND" || op == "OR" => {
            let (expresion_izq, _) = where_clause_rec(lista)?;
            let (expresion_der, rest) = where_clause_rec(rest)?;
            match (expresion_izq, expresion_der) {
                (Some(expr_izq), Some(expr_der)) => Ok((Some(ExpresionBooleana::And(Box::new(expr_izq), Box::new(expr_der))), rest)),
                _ => Err(ErrorType::InvalidSyntax("Operador lógico sin expresión válida.".to_string())),
            }
        },
        // [NOT, ...] manejar el operador NOT
        [Operador::String(not), rest @ ..] if not == "NOT" => procesar_not(rest),
        _ => Err(ErrorType::InvalidSyntax("Sintaxis inválida en el WHERE.".to_string()))
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
fn order_by_rec(rest: &[Operador], mut order_by: Vec<OrderClause>) -> Result<(Vec<OrderClause>, &[Operador]), ErrorType> {
    match rest {
        [] => Ok((order_by, rest)),
        // [columna, direccion, rest]
        [Operador::String(column) | Operador::Texto(column), Operador::String(direccion), rest @ ..]
        if direccion == "ASC" || direccion == "DESC" => {
            let direccion = string_to_direccion(&direccion)?;
            let order_clause = OrderClause {
                column: column.to_string(),
                direccion
            };
            order_by.push(order_clause);
            Ok(order_by_rec(rest, order_by)?)
        }
        // [columna, rest]
        [Operador::String(column) | Operador::Texto(column), rest@..] => {
            let order_clause = OrderClause {
                column: column.to_string(),
                direccion: OrderDirection::Asc
            };
            order_by.push(order_clause);
            Ok(order_by_rec(rest, order_by)?)
        }
        _ => Err(ErrorType::InvalidSyntax("Variables invalidas en WHERE_CLAUSE".to_string())),
    }
}

/// Procesa la lista de operadores para extraer las columnas a ordenar.
fn order_by(rest: &[Operador]) -> Result<(Option<Vec<OrderClause>>, &[Operador]), ErrorType> {
    match rest {
        [Operador::String(order), Operador::String(by), rest @ ..]
        if order == "ORDER" && by == "BY" => {
            match rest {
                [] => Err(ErrorType::InvalidSyntax("Faltan valores despues del ORDER BY ".to_string())),
                _ => {
                    let (order_clauses, rest) = order_by_rec(rest, Vec::new())?;
                    Ok((Some(order_clauses), rest))
                }
            }
        }
        _ => Ok((None, rest)), // ya que no es un campo obligatorio
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
    let (changes, rest) = changes_rec(HashMap::new(), rest)?;
    if changes.is_empty() { return Err(ErrorType::InvalidSyntax("Faltan columnas y valores en la consulta UPDATE.".to_string())); }
    let (where_condition, rest ) = where_clause(rest)?;
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
    if columns.is_empty() {return Err(ErrorType::InvalidSyntax("Sin columnas seleccioanadas en SELECT".to_string()));}
    let (table, rest) = match rest {
        [Operador::String(table) | Operador::Texto(table), rest @ ..] => (table, rest),
        _ => return Err(ErrorType::InvalidSyntax("Se esperaba el nombre de la tabla en SELECT".to_string())),
    };
    let (where_condition, rest ) = where_clause(rest)?;
    let (order_by, rest) = order_by(rest)?;
    if !rest.is_empty() {return Err(ErrorType::InvalidSyntax("Sintaxis invalida en SELECT".to_string()));}
    Ok(SelectQuery::new(columns, table.to_string(), where_condition, order_by))
    
}

/// Procesar INSERT
fn parse_insert_query(table: &String, columns: &Vec<Operador>, values: &Operador, rest: &[Operador]) -> Result<SQLQuery, ErrorType> {
    match values {
        Operador::String(value) if value == "VALUES" => {
            let insert_query = parser_insert(table, columns, rest)?;
            Ok(SQLQuery::Insert(insert_query))
        },
        _ => Err(ErrorType::InvalidSyntax("Falta 'VALUES' en la consulta INSERT.".to_string())),
    }
}

/// Procesar UPDATE
fn parse_update_query(table: &String, rest: &[Operador]) -> Result<SQLQuery, ErrorType> {
    match rest.get(0) {
        Some(Operador::String(set)) if set == "SET" => {
            let update_query = parser_update(table, &rest[1..])?;
            Ok(SQLQuery::Update(update_query))
        },
        _ => Err(ErrorType::InvalidSyntax("Falta 'SET' en la consulta UPDATE.".to_string())),
    }
}

/// Procesar DELETE
fn parse_delete_query(table: &String, rest: &[Operador]) -> Result<SQLQuery, ErrorType> {
    let delete_query = parser_delete(table, rest)?;
    Ok(SQLQuery::Delete(delete_query))
}

/// Procesar SELECT
fn parse_select_query(rest: &[Operador]) -> Result<SQLQuery, ErrorType> {
    let select_query = parser_select(rest)?;
    Ok(SQLQuery::Select(select_query))
}

/// Recibe un string e intenta matchearlo con una Query valida, sino devuelve el error
pub fn parser(query: &Vec<Operador>) -> Result<SQLQuery, ErrorType> {
    match query.as_slice() {
        // [INSERT, INTO, tabla, columns, VALUES, values1, values2 ...]
        [Operador::String(insert), Operador::String(into), Operador::String(table) | Operador::Texto(table), Operador::Lista(columns), values, rest @ ..]
        if insert == "INSERT" && into == "INTO" => {
            parse_insert_query(table, &columns, values, rest)
        },
        // [UPDATE, tabla, SET, column1, valor1, column1, valor1, ..., WHERE, ...]
        [Operador::String(update), Operador::String(table), rest @ ..]
        if update == "UPDATE" => {
            parse_update_query(table, rest)
        },
        // [DELETE, FROM, tabla, WHERE, ...]
        [Operador::String(delete), Operador::String(from), Operador::String(table) | Operador::Texto(table), rest @ ..]
        if delete == "DELETE" && from == "FROM" => {
            parse_delete_query(table, rest)
        },
        // [SELECT, ..., FROM, tabla, WHERE, ..., ORDER, BY, ...]
        [Operador::String(select), rest @ ..]
        if select == "SELECT" => {
            parse_select_query(rest)
        },
        _ => Err(ErrorType::InvalidSyntax("Query invalida".to_string())),
    }
}






#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{dato::Datos, lexer::operador::Operador, queries::{delete_query::DeleteQuery, insert_query::InsertQuery, order_clause::{OrderClause, OrderDirection}, select_query::SelectQuery, sql_query::SQLQuery, update_query::UpdateQuery, where_clause::{expresion_booleana::ExpresionBooleana, operador_comparacion::OperadorComparacion, valor::Valor}}};
    use super::parser;

    // Función auxiliar para probar el parser con un caso de prueba exitoso
    fn probar_parser_exitoso(caso: &Vec<Operador>, esperado: SQLQuery) {
        let resultado = parser(caso);
        match resultado {
            Ok(query) => assert_eq!(query, esperado, "Resultado inesperado para el caso: {:?}", caso),
            Err(e) => println!("Parser devolvió un error inesperado: {} para el caso: {:?}", e.to_string(), caso),
        }
    }

    // Función auxiliar para probar el parser con un caso de prueba que debería fallar
    fn probar_parser_error(caso: &Vec<Operador>, mensaje_error_esperado: &str) {
        let resultado = parser(caso);
        match resultado {
            Ok(_) => println!("Se esperaba un error para el caso: {:?}", caso),
            Err(e) => assert!(e.to_string().contains(mensaje_error_esperado), 
                            "Se esperaba un error que contenga '{}', pero se obtuvo: '{}' para el caso: {:?}", 
                            mensaje_error_esperado, 
                            e.to_string(), 
                            caso),
        }
    }

    #[test]
    fn test_parser_insert_simple() {
        let input = vec![
            Operador::String("INSERT".to_string()),
            Operador::String("INTO".to_string()),
            Operador::String("users".to_string()),
            Operador::Lista(vec![
                Operador::String("id".to_string()),
                Operador::String("name".to_string()),
            ]),
            Operador::String("VALUES".to_string()),
            Operador::Lista(vec![
                Operador::String("1".to_string()),
                Operador::Texto("Ivan".to_string()),
            ]),
        ];
        let mut values_map = HashMap::new();
        values_map.insert("id".to_string(), Datos::Integer(1));
        values_map.insert("name".to_string(), Datos::String("Ivan".to_string()));

        let esperado = SQLQuery::Insert(InsertQuery {
            table: "users".to_string(),
            columns: vec!["id".to_string(), "name".to_string()],
            values: vec![values_map],
        });
        probar_parser_exitoso(&input, esperado);
        println!("Parser insert simple ejecutado correctamente!");
    }

    #[test]
    fn test_parser_insert_complejo() {
        let input = vec![
            Operador::String("INSERT".to_string()),
            Operador::String("INTO".to_string()),
            Operador::String("users".to_string()),
            Operador::Lista(vec![
                Operador::String("id".to_string()),
                Operador::String("name".to_string()),
            ]),
            Operador::String("VALUES".to_string()),
            Operador::Lista(vec![
                Operador::Lista(vec![
                    Operador::Lista(vec![
                        Operador::String("1".to_string()),
                    ]),
                ]),
                Operador::Texto("Ivan Maximoff".to_string()),
            ]),
            Operador::Lista(vec![
                Operador::Lista(vec![
                    Operador::String("2".to_string()),
                ]),
                Operador::Lista(vec![
                    Operador::Lista(vec![
                        Operador::Texto("Alexis".to_string()),
                    ]),
                ]),
            ]),
        ];
        let mut values1 = HashMap::new();
        values1.insert("id".to_string(), Datos::Integer(1));
        values1.insert("name".to_string(), Datos::String("Ivan Maximoff".to_string()));

        let mut values2 = HashMap::new();
        values2.insert("id".to_string(), Datos::Integer(2));
        values2.insert("name".to_string(), Datos::String("Alexis".to_string()));

        let esperado = SQLQuery::Insert(InsertQuery {
            table: "users".to_string(),
            columns: vec!["id".to_string(), "name".to_string()],
            values: vec![values1, values2],
        });

        probar_parser_exitoso(&input, esperado);
        println!("Parser insert complejo ejecutado correctamente!");
    }

    #[test]
    fn test_parser_select_simple() {
        let input = vec![
            Operador::String("SELECT".to_string()),
            Operador::String("id".to_string()),
            Operador::String("name".to_string()),
            Operador::String("FROM".to_string()),
            Operador::String("users".to_string()),
            Operador::String("WHERE".to_string()),
            Operador::String("age".to_string()),
            Operador::Comparador(">".to_string()),
            Operador::String("30".to_string()),
        ];

        let esperado = SQLQuery::Select(SelectQuery {
            columns_select: vec!["id".to_string(), "name".to_string()],
            table: "users".to_string(),
            where_clause: Some(ExpresionBooleana::Comparacion {
                izq: Valor::String("age".to_string()),
                operador: OperadorComparacion::Mayor,
                der: Valor::String("30".to_string()),
            }),
            order_by: None,
        });

        probar_parser_exitoso(&input, esperado);
        println!("Parser select simple ejecutado correctamente!");
    }
    
    #[test]
    fn test_parser_select_complejo() {
        let input = vec![
            Operador::String("SELECT".to_string()),
            Operador::String("id".to_string()),
            Operador::String("name".to_string()),
            Operador::String("FROM".to_string()),
            Operador::Texto("users".to_string()),
            Operador::String("WHERE".to_string()),
            Operador::Lista(vec![
                Operador::Texto("age".to_string()),
                Operador::Comparador(">".to_string()),
                Operador::String("30".to_string()),
            ]),
            Operador::String("AND".to_string()),
            Operador::Lista(vec![
                Operador::String("status".to_string()),
                Operador::Comparador("=".to_string()),
                Operador::Texto("active".to_string()),
            ]),
            Operador::String("ORDER".to_string()),
            Operador::String("BY".to_string()),
            Operador::String("name".to_string()),
            Operador::String("DESC".to_string()),
        ];
    
        let esperado = SQLQuery::Select(SelectQuery {
            columns_select: vec!["id".to_string(), "name".to_string()],
            table: "users".to_string(),
            where_clause: Some(ExpresionBooleana::And(
                Box::new(ExpresionBooleana::Comparacion {
                    izq: Valor::Literal("age".to_string()),
                    operador: OperadorComparacion::Mayor,
                    der: Valor::String("30".to_string()),
                }),
                Box::new(ExpresionBooleana::Comparacion {
                    izq: Valor::String("status".to_string()),
                    operador: OperadorComparacion::Igual,
                    der: Valor::Literal("active".to_string()),
                }),
            )),
            order_by: Some(vec![OrderClause {
                column: "name".to_string(),
                direccion: OrderDirection::Desc,
            }]),
        });
    
        probar_parser_exitoso(&input, esperado);
        println!("Parser select complejo ejecutado correctamente!");
    }

    #[test]
    fn test_parser_update_simple() {
        let input = vec![
            Operador::String("UPDATE".to_string()),
            Operador::String("users".to_string()),
            Operador::String("SET".to_string()),
            Operador::String("name".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::Texto("Ivan".to_string()),
            Operador::String("WHERE".to_string()),
            Operador::String("id".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::String("1".to_string()),
        ];
        
        let mut changes = HashMap::new();
        changes.insert("name".to_string(), Datos::String("Ivan".to_string()));

        let where_condition: Option<ExpresionBooleana> = Some(ExpresionBooleana::Comparacion {
            izq: Valor::String("id".to_string()), 
            operador: OperadorComparacion::Igual,
            der: Valor::String("1".to_string()),
        });

        let esperado = SQLQuery::Update(UpdateQuery {
            table: "users".to_string(),
            changes,
            where_condition,
        });
        probar_parser_exitoso(&input, esperado);
        println!("Parser update simple ejecutado correctamente!");
    }

    #[test]
    fn test_parser_update_complejo() {
        let input = vec![
            Operador::String("UPDATE".to_string()),
            Operador::String("users".to_string()),
            Operador::String("SET".to_string()),
            Operador::String("name".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::Texto("Ivan".to_string()),
            Operador::String("age".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::String("30".to_string()),
            Operador::String("WHERE".to_string()),
            Operador::String("id".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::String("1".to_string()),
            Operador::String("AND".to_string()),
            Operador::String("status".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::Texto("active".to_string()),
        ];

        let mut changes = HashMap::new();
        changes.insert("name".to_string(), Datos::String("Ivan".to_string()));
        changes.insert("age".to_string(), Datos::Integer(30));

        let where_condition = Some(ExpresionBooleana::And(
            Box::new(ExpresionBooleana::Comparacion {
                izq: Valor::String("id".to_string()),
                operador: OperadorComparacion::Igual,
                der: Valor::String("1".to_string()),
            }),
            Box::new(ExpresionBooleana::Comparacion {
                izq: Valor::String("status".to_string()),
                operador: OperadorComparacion::Igual,
                der: Valor::Literal("active".to_string()),
            }),
        ));

        let esperado = SQLQuery::Update(UpdateQuery {
            table: "users".to_string(),
            changes,
            where_condition,
        });
        probar_parser_exitoso(&input, esperado);
        println!("Parser update complejo ejecutado correctamente!");
    }

    #[test]
    fn test_parser_delete_simple() {
        let input = vec![
            Operador::String("DELETE".to_string()),
            Operador::String("FROM".to_string()),
            Operador::String("users".to_string()),
            Operador::String("WHERE".to_string()),
            Operador::String("id".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::String("1".to_string()),
        ];
        let where_clause = Some(ExpresionBooleana::Comparacion {
            izq: Valor::String("id".to_string()),
            operador: OperadorComparacion::Igual,
            der: Valor::String("1".to_string()),
        });
        let esperado = SQLQuery::Delete(DeleteQuery {
            table: "users".to_string(),
            where_clause,
        });
        probar_parser_exitoso(&input, esperado);
    }
    
    #[test]
    fn test_parser_delete_complejo() {
        let input = vec![
            Operador::String("DELETE".to_string()),
            Operador::String("FROM".to_string()),
            Operador::String("users".to_string()),
            Operador::String("WHERE".to_string()),
            Operador::Lista(vec![
                Operador::String("id".to_string()),
                Operador::Comparador("=".to_string()),
                Operador::String("1".to_string()),
                Operador::String("OR".to_string()),
                Operador::String("id".to_string()),
                Operador::Comparador("=".to_string()),
                Operador::String("2".to_string()),
            ]),
            Operador::String("AND".to_string()),
            Operador::String("status".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::Texto("inactive".to_string()),
        ];
        let where_clause = Some(
            ExpresionBooleana::And(
                Box::new(
                    ExpresionBooleana::Or(
                        Box::new(
                            ExpresionBooleana::Comparacion {
                                izq: Valor::String("id".to_string()),
                                operador: OperadorComparacion::Igual,
                                der: Valor::String("1".to_string()),
                            }
                        ),
                        Box::new(
                            ExpresionBooleana::Comparacion {
                                izq: Valor::String("id".to_string()),
                                operador: OperadorComparacion::Igual,
                                der: Valor::String("2".to_string()),
                            }
                        )
                    )
                ),
                Box::new(
                    ExpresionBooleana::Comparacion {
                        izq: Valor::String("status".to_string()),
                        operador: OperadorComparacion::Igual,
                        der: Valor::Literal("inactive".to_string()),
                    }
                )
            )
        );
        let esperado = SQLQuery::Delete(DeleteQuery {
            table: "users".to_string(),
            where_clause,
        });
        probar_parser_exitoso(&input, esperado);
    }

    #[test]
    fn test_parser_error_insert_falta_values() {
        let input = vec![
            Operador::String("INSERT".to_string()),
            Operador::String("INTO".to_string()),
            Operador::String("users".to_string()),
            Operador::Lista(vec![
                Operador::String("id".to_string()),
                Operador::String("name".to_string()),
            ]),
            Operador::Lista(vec![
                Operador::String("4".to_string()),
                Operador::String("Ivan".to_string()),
            ]),
        ];
        let error = "Falta 'VALUES' en la consulta INSERT.".to_string();
        probar_parser_error(&input, &error);
    }

    #[test]
    fn test_parser_error_update_falta_set() {
        let input = vec![
            Operador::String("UPDATE".to_string()),
            Operador::String("users".to_string()),
        ];
        let error = "Falta 'SET' en la consulta UPDATE.".to_string();
        probar_parser_error(&input, &error);
    }
}