use std::env;

use tp1::{error::ErrorType, executer::execute_query, lexer::{lexer, Operador}, parser::parser};

static POS_PATH: usize = 1;
static POS_QUERY: usize = 2;

fn entrada_valida(args: &Vec<String>)-> bool{
    if args.len() < 3 {
        println!("{}", ErrorType::Error("Cantidad de argumentos invalido.".to_string()).to_string())
    }
    return args.len() >= 3;
}

// Limpia los tabeos, saltos de lineas y espacios en blanco en los extremos de los queries
fn limpiar_entrada(entrada: &String)->Vec<String>{
    let limpiado = entrada
        .trim()
        .replace('\n', " ");
    let limpiado: Vec<String> = limpiado.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    limpiado
}

fn main() {
    let args: Vec<String> = env::args().collect(); // args = [ruta, consulta]
    if !entrada_valida(&args){
        return;
    }
    let texto: Vec<String> = limpiar_entrada(&args[POS_QUERY]);// ["Operacion1 ...", "Operacion2 ...", ...]
    for query in &texto{
        let query: Vec<Operador> = match lexer(query){
            Ok(query) => query,
            Err(e) => {
                println!("{}", e.to_string());
                Vec::new()
            }
        };
        match parser(query){
            Ok(parsed_query) => execute_query(&args[POS_PATH], parsed_query), 
            Err(e) => println!("{}", e.to_string()),
        }
    }
}
