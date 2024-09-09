#[cfg(test)]
mod integration_tests {
    use tp1::dato::Datos;
    use tp1::lexer::{lexer, Operador};
    use tp1::parser::parser;
    use tp1::queries::comparadores::{ExpresionBooleana, OperadorComparacion};
    use tp1::queries::SQLQuery;
    #[test]
    fn test_insert_query() {
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
        let expected_values = vec![
            vec![
                Datos::Integer(111),
                Datos::Integer(6),
                Datos::String("laptop hola".to_string()),
                Datos::Integer(3)
            ]
        ];

        match parser(query) {
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
                    _ =>  panic!("La prueba falló al matchear con INSERT")
                }
            },
            Err(e) => panic!("La prueba falló con el error: {}", e.to_string())
        }
    }

    #[test]
    fn test_update_query() {
        let input = "UPDATE clientes SET nombre = 'Juan', edad = 30 WHERE id = 1".to_string();
        let query: Vec<Operador> = match lexer(&input) {
            Ok(query) => query,
            Err(e) => {
                println!("{}", e.to_string());
                Vec::new()
            }
        };
    
        let table = "clientes".to_string();
        let expected_changes = vec![
            ("nombre".to_string(), Datos::String("Juan".to_string())),
            ("edad".to_string(), Datos::Integer(30))
        ];
        let expected_where_clause = Some(ExpresionBooleana::Comparacion {
            izq: "id".to_string(),
            operador: OperadorComparacion::Igual,
            der: "1".to_string(),
        });
    
        match parser(query) {
            Ok(SQLQuery::Update(update_query)) => {
                assert_eq!(update_query.table, table);
                assert_eq!(update_query.changes, expected_changes);
    
                // Comparación correcta para `Option<ExpresionBooleana>`
                assert_eq!(update_query.where_condition, expected_where_clause);
    
                println!("Update query fue exitoso");
            },
            Ok(_) => panic!("La prueba falló al matchear con UPDATE"),
            Err(e) => panic!("La prueba falló con el error: {}", e.to_string())
        }
    }

    #[test]
    fn test_delete_query() {
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
            izq: "edad".to_string(),
            operador: OperadorComparacion::Menor,
            der: "18".to_string(),
        });

        match parser(query) {
            Ok(SQLQuery::Delete(delete_query)) => {
                assert_eq!(delete_query.table, table);
                assert_eq!(delete_query.where_clause, expected_where_clause);
                println!("Delete query fue exitoso");
            },
            Ok(_) => panic!("La prueba falló al matchear con DELETE"),
            Err(e) => panic!("La prueba falló con el error: {}", e.to_string())
        }
    }

    #[test]
    fn test_select_query() {
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
            izq: "edad".to_string(),
            operador: OperadorComparacion::Mayor,
            der: "25".to_string(),
        });
        let expected_order = "ASC".to_string();
        let expected_by = vec!["nombre".to_string()];

        match parser(query) {
            Ok(SQLQuery::Select(select_query)) => {
                assert_eq!(select_query.columns, expected_columns);
                assert_eq!(select_query.table, expected_table);
                assert_eq!(select_query.where_clause, expected_where_clause);
                assert_eq!(select_query.order, expected_order);
                assert_eq!(select_query.by, expected_by);
                println!("Select query fue exitoso");
            },
            Ok(_) => panic!("La prueba falló al matchear con SELECT"),
            Err(e) => panic!("La prueba falló con el error: {}", e.to_string())
        }
    }
}