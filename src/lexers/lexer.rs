use crate::errores::error::ErrorType;

use super::operador::Operador;

/// Agrega operador a operadores o si hay a la lista
fn agregar_operador(
    operadores: &mut Vec<Operador>,
    lista: &mut Vec<Operador>,
    lista_open: bool,
    operador: Operador,
) {
    if lista_open {
        lista.push(operador);
    } else {
        operadores.push(operador);
    }
}

/// Agrega el string como operador a operadores o si hay una lista si el string no esta vacio
fn agregar_substring(
    operadores: &mut Vec<Operador>,
    lista: &mut Vec<Operador>,
    lista_open: bool,
    substring: &mut String,
) {
    if !substring.is_empty() {
        agregar_operador(
            operadores,
            lista,
            lista_open,
            Operador::String(substring.to_string()),
        );
        substring.clear();
    }
}

/// Recibe un texto entre comillas simples y lo devuelve con espacios simples entre cada palabra
fn string_rec(
    string: &String,
    actual: usize,
    mut substring: String,
    espacio: bool,
) -> Result<(String, usize), ErrorType> {
    if actual >= string.len() {
        dbg!(string);
        return Err(ErrorType::InvalidSyntax(
            "Comilla simple sin cerrar.".to_string(),
        ));
    }
    let Some(caracter) = string.chars().nth(actual) else {
        return Err(ErrorType::InvalidSyntax("Fallo".to_string()));
    };
    match caracter {
        '\'' => Ok((substring, actual)),
        ' ' => string_rec(string, actual + 1, substring, true),
        _ => {
            if espacio && !substring.is_empty() {
                substring.push(' ');
            }
            substring.push_str(&caracter.to_string());
            string_rec(string, actual + 1, substring, false)
        }
    }
}

/// Funcion recursiva que matchea todos los caracteres y crea los operadores
fn lexer_rec(
    string: &String,
    actual: usize,
    mut operadores: Vec<Operador>,
    mut substring: String,
    mut lista: Vec<Operador>,
    lista_open: bool,
) -> Result<(Vec<Operador>, usize), ErrorType> {
    if actual >= string.chars().count() {
        // caso base: termino el string
        if lista_open {
            return Err(ErrorType::InvalidSyntax(
                "Falta cerrar parentesis.".to_string(),
            ));
        }
        agregar_substring(&mut operadores, &mut lista, lista_open, &mut substring);
        return Ok((operadores, actual));
    }
    let Some(caracter) = string.chars().nth(actual) else {
        return Err(ErrorType::InvalidSyntax("Fallo".to_string()));
    };
    match caracter {
        '\'' => {
            // hay un "texto"
            let (substring, actual) = string_rec(string, actual + 1, "".to_string(), false)?;
            agregar_operador(
                &mut operadores,
                &mut lista,
                lista_open,
                Operador::Texto(substring),
            );
            lexer_rec(
                string,
                actual + 1,
                operadores,
                "".to_string(),
                lista,
                lista_open,
            )
        }
        '(' => {
            // empieza una lista
            agregar_substring(&mut operadores, &mut lista, lista_open, &mut substring);
            let (nueva_lista, nuevo_actual) = lexer_rec(
                string,
                actual + 1,
                Vec::new(),
                "".to_string(),
                Vec::new(),
                true,
            )?;
            for elemento in nueva_lista {
                // siempre va a ser uno solo
                agregar_operador(&mut operadores, &mut lista, lista_open, elemento);
            }
            lexer_rec(
                string,
                nuevo_actual + 1,
                operadores,
                substring,
                lista,
                lista_open,
            )
        }
        ')' => {
            // termino una lista
            if !lista_open {
                return Err(ErrorType::InvalidSyntax(
                    "Paréntesis de cierre sin haber uno de apertura.".to_string(),
                ));
            }
            if !substring.is_empty() {
                lista.push(Operador::String(substring))
            }
            if lista.is_empty() {
                return Err(ErrorType::InvalidSyntax(
                    "Paréntesis sin nada adentro".to_string(),
                ));
            }
            operadores.push(Operador::Lista(lista));
            Ok((operadores, actual))
        }
        '=' | '>' | '<' => {
            // caracter especial que agrega la palabra anterior y luego se pushea a el mismo
            let mut operador = caracter.to_string();
            if (caracter == '>' || caracter == '<') && actual + 1 < string.len() {
                let siguiente = &string[actual + 1..actual + 2];
                if siguiente == "=" {
                    operador.push_str(siguiente);
                }
            }
            let actual = actual + operador.len();
            agregar_substring(&mut operadores, &mut lista, lista_open, &mut substring);
            agregar_operador(
                &mut operadores,
                &mut lista,
                lista_open,
                Operador::Comparador(operador),
            );
            lexer_rec(
                string,
                actual,
                operadores,
                "".to_string(),
                lista,
                lista_open,
            )
        }
        ' ' | ',' if substring.is_empty() => {
            lexer_rec(string, actual + 1, operadores, substring, lista, lista_open)
        }
        ' ' | ',' => {
            // termino una palabra no vacia
            agregar_substring(&mut operadores, &mut lista, lista_open, &mut substring);
            lexer_rec(
                string,
                actual + 1,
                operadores,
                "".to_string(),
                lista,
                lista_open,
            )
        }
        _ => {
            // agrego el caracter
            substring.push_str(&caracter.to_string());
            lexer_rec(string, actual + 1, operadores, substring, lista, lista_open)
        }
    }
}

/// Recibe un string y lo convierte en un vector de operadores validos
pub fn lexer(string: &String) -> Result<Vec<Operador>, ErrorType> {
    let (operadores, _) = lexer_rec(string, 0, Vec::new(), "".to_string(), Vec::new(), false)?;
    Ok(operadores)
}

// TEST lexer unitarios

#[cfg(test)]
mod tests {
    use super::lexer;
    use crate::lexers::operador::Operador;

    /// Función auxiliar para probar el lexer con un caso de prueba exitoso
    fn probar_lexer_exitoso(caso: &String, esperado: Vec<Operador>) {
        let resultado = lexer(&caso);
        match resultado {
            Ok(operadores) => {
                assert_eq!(
                    operadores.len(),
                    esperado.len(),
                    "Número de operadores incorrecto para el caso: {}",
                    caso
                );
                for i in 0..esperado.len() {
                    assert_eq!(
                        operadores.get(i),
                        esperado.get(i),
                        "Error en el operador en el índice {} para el caso: {}",
                        i,
                        caso
                    );
                }
            }
            Err(e) => println!(
                "Lexer devolvió un error inesperado: {} para el caso: {}",
                e.to_string(),
                caso
            ),
        }
    }

    /// Función auxiliar para probar el lexer con un caso de prueba que debería fallar
    fn probar_lexer_error(caso: &String, mensaje_error_esperado: &String) {
        let resultado = lexer(&caso);
        match resultado {
            Ok(_) => println!("Se esperaba un error para el caso: {}", caso),
            Err(e) => assert!(
                e.to_string().contains(mensaje_error_esperado),
                "Se esperaba un error que contenga '{}', pero se obtuvo: '{}' para el caso: {}",
                mensaje_error_esperado,
                e.to_string(),
                caso
            ),
        }
    }

    #[test]
    fn test_lexer_insert_simple() {
        let input = "INSERT INTO users (id, name) VALUES (1, 'Ivan')".to_string();
        let esperado = vec![
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
        probar_lexer_exitoso(&input, esperado);
        println!("Lexer insert simple ejecutado correctamente!");
    }

    #[test]
    fn test_lexer_insert_complejo() {
        let input = "INSERT INTO   users ( id   ,name)    VALUES (((1)) , '  Ivan    Maximoff     '), ((2), (('Alexis')))".to_string();
        let esperado = vec![
            Operador::String("INSERT".to_string()),
            Operador::String("INTO".to_string()),
            Operador::String("users".to_string()),
            Operador::Lista(vec![
                Operador::String("id".to_string()),
                Operador::String("name".to_string()),
            ]),
            Operador::String("VALUES".to_string()),
            Operador::Lista(vec![
                Operador::Lista(vec![Operador::Lista(vec![Operador::String(
                    "1".to_string(),
                )])]),
                Operador::Texto("Ivan Maximoff".to_string()),
            ]),
            Operador::Lista(vec![
                Operador::Lista(vec![Operador::String("2".to_string())]),
                Operador::Lista(vec![Operador::Lista(vec![Operador::Texto(
                    "Alexis".to_string(),
                )])]),
            ]),
        ];
        probar_lexer_exitoso(&input, esperado);
        println!("Lexer insert complejo ejecutado correctamente!");
    }

    #[test]
    fn test_lexer_select_simple() {
        let input = "SELECT id, name FROM users WHERE age > 30".to_string();
        let esperado = vec![
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
        probar_lexer_exitoso(&input, esperado);
        println!("Lexer select simple ejecutado correctamente!");
    }

    #[test]
    fn test_lexer_select_complejo() {
        let input =
            "SELECT id, name FROM 'users' WHERE ('age' >= 30) AND (status = 'active')".to_string();
        let esperado = vec![
            Operador::String("SELECT".to_string()),
            Operador::String("id".to_string()),
            Operador::String("name".to_string()),
            Operador::String("FROM".to_string()),
            Operador::Texto("users".to_string()),
            Operador::String("WHERE".to_string()),
            Operador::Lista(vec![
                Operador::Texto("age".to_string()),
                Operador::Comparador(">=".to_string()),
                Operador::String("30".to_string()),
            ]),
            Operador::String("AND".to_string()),
            Operador::Lista(vec![
                Operador::String("status".to_string()),
                Operador::Comparador("=".to_string()),
                Operador::Texto("active".to_string()),
            ]),
        ];
        probar_lexer_exitoso(&input, esperado);
        println!("Lexer select complejo ejecutado correctamente!");
    }

    #[test]
    fn test_lexer_update_simple() {
        let input = "UPDATE users SET name = 'Ivan' WHERE id = 1".to_string();
        let esperado = vec![
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
        probar_lexer_exitoso(&input, esperado);
        println!("Lexer update simple ejecutado correctamente!");
    }

    #[test]
    fn test_lexer_update_complejo() {
        let input = "UPDATE users SET name = 'Ivan', age = 30 WHERE id = 1 AND status = 'active'"
            .to_string();
        let esperado = vec![
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
        probar_lexer_exitoso(&input, esperado);
        println!("Lexer update complejo ejecutado correctamente!");
    }

    #[test]
    fn test_lexer_delete_simple() {
        let input = "DELETE FROM users WHERE id = 1".to_string();
        let esperado = vec![
            Operador::String("DELETE".to_string()),
            Operador::String("FROM".to_string()),
            Operador::String("users".to_string()),
            Operador::String("WHERE".to_string()),
            Operador::String("id".to_string()),
            Operador::Comparador("=".to_string()),
            Operador::String("1".to_string()),
        ];
        probar_lexer_exitoso(&input, esperado);
        println!("Lexer delete simple ejecutado correctamente!");
    }

    #[test]
    fn test_lexer_delete_complejo() {
        let input =
            "DELETE FROM users WHERE (id = 1 OR id = 2) AND status = 'inactive'".to_string();
        let esperado = vec![
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
        probar_lexer_exitoso(&input, esperado);
        println!("Lexer delete complejo ejecutado correctamente!");
    }

    #[test]
    fn test_lexer_error_parentesis_sin_cerrar() {
        let input = "(1, 'Alice'".to_string();
        let error = "Falta cerrar parentesis.".to_string();
        probar_lexer_error(&input, &error);
        println!("Error en parentesis encontrado correctamente!");
    }

    #[test]
    fn test_lexer_error_parentesis_sin_abrir() {
        let input = "1, 'Alice')".to_string();
        let error = "Paréntesis de cierre sin haber uno de apertura.".to_string();
        probar_lexer_error(&input, &error);
        println!("Error en parentesis encontrado correctamente!");
    }

    #[test]
    fn test_lexer_error_parentesis_vacio() {
        let input = "()".to_string();
        let error = "Paréntesis sin nada adentro".to_string();
        probar_lexer_error(&input, &error);
        println!("Error en parentesis encontrado correctamente!");
    }

    #[test]
    fn test_lexer_error_cerrar_texto() {
        let input = "'Ivan".to_string();
        let error = "Comilla simple sin cerrar.".to_string();
        probar_lexer_error(&input, &error);
        println!("Error en comillas simples encontrado correctamente!");
    }
}
