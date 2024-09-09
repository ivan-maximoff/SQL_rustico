use crate::errores::error::ErrorType;

use super::operador::Operador;

/// Agrega operador a operadores o si hay a la lista
fn agregar_operador(operadores: &mut Vec<Operador>, lista: &mut Vec<Operador>, lista_open: bool, operador: Operador,) {
    if lista_open {
        lista.push(operador);
    } else {
        operadores.push(operador);
    }
}

/// Agrega el string como operador a operadores o si hay una lista si el string no esta vacio
fn agregar_substring(operadores: &mut Vec<Operador>, lista: &mut Vec<Operador>, lista_open: bool, substring: &mut String,) {
    if !substring.is_empty() {
        agregar_operador(operadores, lista, lista_open, Operador::String(substring.to_string()));
        substring.clear();
    }
}

/// Recibe un texto entre comillas simples y lo devuelve con espacios simples entre cada palabra
fn string_rec(string: &String, actual: usize, mut substring: String, espacio: bool) -> (String, usize){
    if actual >= string.len() { return (substring.to_string(), actual);}
    let caracter = &string[actual..actual+1];
    match caracter{
        "\'" => (substring, actual),
        " " => string_rec(string, actual+1, substring, true), 
        _ => {
            if espacio {substring.push_str(" ");}
            substring.push_str(caracter);
            string_rec(string, actual+1, substring, false)
        }
    }
}

/// Funcion recursiva que matchea todos los caracteres y crea los operadores
fn lexer_rec(string: &String, actual: usize, mut operadores: Vec<Operador>, mut substring: String, mut lista: Vec<Operador>, lista_open: bool) ->  Result<(Vec<Operador>, usize), ErrorType> {
    if actual >= string.len() { // caso base: termino el string
        if lista_open {return Err(ErrorType::InvalidSyntax("Falta cerrar parentesis.".to_string()));}
        agregar_substring(&mut operadores, &mut lista, lista_open, &mut substring);
        return Ok((operadores, actual));
    }
    let caracter = &string[actual..actual+1];
    match caracter{
        "\'" => { // hay un "texto"
            let (substring, actual) = string_rec(string, actual+1, "".to_string(), false);
            agregar_operador(&mut operadores, &mut lista, lista_open, Operador::Texto(substring));
            lexer_rec(string, actual+1, operadores, "".to_string(), lista, lista_open)
        }
        "(" => { // empieza una lista
            agregar_substring(&mut operadores, &mut lista, lista_open, &mut substring);
            let (nueva_lista, nuevo_actual) = lexer_rec(string, actual + 1, Vec::new(), "".to_string(), Vec::new(), true)?;
            for elemento in nueva_lista{ // siempre va a ser uno solo
                agregar_operador(&mut operadores, &mut lista, lista_open, elemento);
            }
            lexer_rec(string, nuevo_actual + 1, operadores, substring, lista, lista_open)
        }
        ")" =>{ // termino una lista
            if !substring.is_empty(){lista.push(Operador::String(substring))}
            if !lista_open {return Err(ErrorType::InvalidSyntax("Paréntesis de cierre sin haber abierto.".to_string()));}
            if lista.len() == 0{
                return Err(ErrorType::InvalidSyntax("Paréntesis sin nada adentro".to_string()));
            }
            operadores.push(Operador::Lista(lista));
            Ok((operadores, actual))
        }
        "=" | ">" | "<" => { // caracter especial que agrega la palabra anterior y luego se pushea a el mismo
            agregar_substring(&mut operadores, &mut lista, lista_open, &mut substring);
            agregar_operador(&mut operadores, &mut lista, lista_open, Operador::Comparador(caracter.to_string()));
            lexer_rec(string, actual+1, operadores, "".to_string(), lista, lista_open)
        }
        " " | "," if substring.is_empty() => lexer_rec(string, actual+1, operadores, substring, lista, lista_open),
        " " | "," => { // termino una palabra no vacia
            agregar_substring(&mut operadores, &mut lista, lista_open, &mut substring);
            lexer_rec(string, actual+1, operadores, "".to_string(), lista, lista_open)
        }
        _ => { // agrego el caracter
            substring.push_str(caracter);
            lexer_rec(string, actual+1, operadores, substring, lista, lista_open)
        }
    }
}

/// Recibe un string y lo convierte en un vector de operadores validos
pub fn lexer(string: &String) -> Result<Vec<Operador>, ErrorType>{
    let(operadores, _) = lexer_rec(string, 0, Vec::new(), "".to_string(), Vec::new(), false)?;
    Ok(operadores)
}

/// Prueba mandar un string y lo convierte en Operadores
#[test]
fn test_lexer(){
    let prueba: String = "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'laptop hola' , 3)".to_string();
    println!("{}", prueba);
    let query = match lexer(&prueba){
        Ok(query) => query,
        Err(e) => {
            println!("{}", e.to_string());
            Vec::new()
        } 
    };

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
    
    for elemento in &query {
        print_operador(elemento, 0);
    }
}