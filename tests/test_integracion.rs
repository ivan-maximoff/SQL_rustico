#[cfg(test)]
mod integration_tests {
    use std::collections::HashMap;

    use tp1::{dato::Datos, lexer::{lexer::lexer, operador::Operador}, parser::parser::parser, queries::{order_clause::{OrderClause, OrderDirection}, sql_query::SQLQuery, where_clause::{expresion_booleana::ExpresionBooleana, operador_comparacion::OperadorComparacion, valor::Valor}}, utils::procesar_consulta};

    /// LEXER Y PARSER
    #[test]
    fn test_parser_insert_query() {
        let input = "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'laptop hola' , 3)".to_string();
        let query: Vec<Operador> = match lexer(&input){
            Ok(query) => query,
            Err(e) => {
                println!("{}", e.to_string());
                Vec::new()
            }
        };

        let table = "ordenes".to_string();
        let expected_columns = vec![
            "id".to_string(),
            "id_cliente".to_string(),
            "producto".to_string(),
            "cantidad".to_string(),
        ];
        let mut expected_values = Vec::new();
        let mut row1 = HashMap::new();
        row1.insert("id".to_string(), Datos::Integer(111));
        row1.insert("id_cliente".to_string(), Datos::Integer(6));
        row1.insert("producto".to_string(), Datos::String("laptop hola".to_string()));
        row1.insert("cantidad".to_string(), Datos::Integer(3));
        expected_values.push(row1);

        match parser(&query) {
            Ok(insert_query) => {
                match insert_query {
                    SQLQuery::Insert(insert_query) => {
                        assert_eq!(insert_query.table, table);
                        assert_eq!(insert_query.columns, expected_columns);
                        assert_eq!(insert_query.values, expected_values);
                        assert_eq!(insert_query.values.len(), 1);
                        assert_eq!(insert_query.values[0].len(), 4);
                        println!("Insert query fue exitoso");
                    },
                    _ =>  println!("La prueba falló al matchear con INSERT")
                }
            },
            Err(e) => println!("La prueba falló con el error: {}", e.to_string())
        }
        
    }

    #[test]
    fn test_parser_update_query() {
        let input = "UPDATE clientes SET nombre = 'Juan', edad = 30 WHERE id = 1".to_string();
        let query: Vec<Operador> = match lexer(&input) {
            Ok(query) => query,
            Err(e) => {
                println!("{}", e.to_string());
                Vec::new()
            }
        };
    
        let table = "clientes".to_string();

        let mut expected_changes: HashMap<String, Datos> = HashMap::new();
        expected_changes.insert("nombre".to_string(), Datos::String("Juan".to_string()));
        expected_changes.insert("edad".to_string(), Datos::Integer(30));

        let expected_where_clause = Some(ExpresionBooleana::Comparacion {
            izq: Valor::String("id".to_string()),
            operador: OperadorComparacion::Igual,
            der: Valor::String("1".to_string()),
        });
    
        match parser(&query) {
            Ok(SQLQuery::Update(update_query)) => {
                assert_eq!(update_query.table, table);
                assert_eq!(update_query.changes, expected_changes);
    
                // Comparación correcta para `Option<ExpresionBooleana>`
                assert_eq!(update_query.where_condition, expected_where_clause);
    
                println!("Update query fue exitoso");
            },
            Ok(_) => println!("La prueba falló al matchear con UPDATE"),
            Err(e) => println!("La prueba falló con el error: {}", e.to_string())
        }
    }

    #[test]
    fn test_parser_delete_query() {
        let input = "DELETE FROM usuarios WHERE edad < 18".to_string();
        let query: Vec<Operador> = match lexer(&input) {
            Ok(query) => query,
            Err(e) => {
                println!("{}", e.to_string());
                Vec::new()
            }
        };

        let table = "usuarios".to_string();
        let expected_where_clause = Some(ExpresionBooleana::Comparacion {
            izq: Valor::String("edad".to_string()),
            operador: OperadorComparacion::Menor,
            der: Valor::String("18".to_string()),
        });

        match parser(&query) {
            Ok(SQLQuery::Delete(delete_query)) => {
                assert_eq!(delete_query.table, table);
                assert_eq!(delete_query.where_clause, expected_where_clause);
                println!("Delete query fue exitoso");
            },
            Ok(_) => println!("La prueba falló al matchear con DELETE"),
            Err(e) => println!("La prueba falló con el error: {}", e.to_string())
        }
    }

    #[test]
    fn test_parser_select_query() {
        let input = "SELECT nombre, edad FROM empleados WHERE edad > 25 ORDER BY nombre ASC".to_string();
        let query: Vec<Operador> = match lexer(&input) {
            Ok(query) => query,
            Err(e) => {
                println!("{}", e.to_string());
                Vec::new()
            }
        };

        let expected_columns = vec!["nombre".to_string(), "edad".to_string()];
        let expected_table = "empleados".to_string();
        let expected_where_clause = Some(ExpresionBooleana::Comparacion {
            izq: Valor::String("edad".to_string()),
            operador: OperadorComparacion::Mayor,
            der: Valor::String("25".to_string()),
        });
        let expected_order_by = Some(vec![OrderClause {
            column: "nombre".to_string(),
            direccion: OrderDirection::Asc,
        }]);

        match parser(&query) {
            Ok(SQLQuery::Select(select_query)) => {
                assert_eq!(select_query.columns_select, expected_columns);
                assert_eq!(select_query.table, expected_table);
                assert_eq!(select_query.where_clause, expected_where_clause);
                assert_eq!(select_query.order_by, expected_order_by);
                println!("Select query fue exitoso");
            },
            Ok(_) => println!("La prueba falló al matchear con SELECT"),
            Err(e) => println!("La prueba falló con el error: {}", e.to_string())
        }
    }

    /// LEXER, PARSER Y EXECUTER
    #[test]
    fn test_insert_query(){
        let input = "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (110, 6, 'laptop hola' , 3), (111, 6, 'laptop hola' , 3)".to_string();
        let path = "../tp1/pruebas".to_string();
        if let Err(e) = procesar_consulta(&input, &path) {
            println!("{}", e.to_string());
        }
    }

    #[test]
    fn test_delete_query() {
        let input = "DELETE FROM ordenes WHERE id <= 111".to_string();
        let path = "../tp1/pruebas".to_string();
        if let Err(e) = procesar_consulta(&input, &path) {
            println!("{}", e.to_string());
        }
    }

    #[test]
    fn test_update_query() {
        let input = "UPDATE ordenes SET id = 116 WHERE id = 110".to_string();
        let path = "../tp1/pruebas".to_string();
        if let Err(e) = procesar_consulta(&input, &path) {
            println!("{}", e.to_string());
        }
    }

    #[test]
    fn test_select_query() {
        let input = "SELECT * FROM ordenes ORDER BY id".to_string();
        let path = "../tp1/pruebas".to_string();
        if let Err(e) = procesar_consulta(&input, &path) {
            println!("{}", e.to_string());
        }}
}