use std::env;

use tp1::{errores::error::ErrorType, utils::procesar_consulta};

static POS_PATH: usize = 1;
static POS_QUERY: usize = 2;

/// Verifica si el número de argumentos de línea de comandos es válido.
fn entrada_valida(args: &Vec<String>)-> bool{
    if args.len() < 3 {
        println!("{}", ErrorType::Error("Cantidad de argumentos invalido.".to_string()).to_string())
    }
    return args.len() >= 3;
}

/// Limpia la entrada eliminando espacios en blanco y saltos de línea, y divide en consultas.
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
        if let Err(e) = procesar_consulta(query, &args[POS_PATH]) {
            eprintln!("{}", e.to_string());
            return;
        }
    }
}
