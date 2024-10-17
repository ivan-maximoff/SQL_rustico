use crate::{dato::Datos, errores::error::ErrorType, executer::execute::Execute, lexers::{lexer::lexer, operador::Operador}, parsers::parser::parser, queries::{order_clause::OrderDirection, where_clause::{operador_comparacion::OperadorComparacion, valor::Valor}}};
/// Procesa una consulta SQL: analiza, convierte y ejecuta.
pub fn procesar_consulta(query: &String, path: &str) -> Result<(), ErrorType> {
    let query_lexer = lexer(query)?;
    let query_parser = parser(&query_lexer)?;
    query_parser.execute(path)
}

/// Imprime la representación de una lista de operadores en formato legible,
/// indentando los niveles de profundidad para listas anidadas.
/// Creado principalmente para validar de forma legible que devuelve de forma correcta
pub fn printear(rest: &[Operador]){
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

// Conversiones entre enums

/// Transforma un string a numero
pub fn string_to_number(s: String) -> Result<Datos, ErrorType> {
    match s.parse::<i64>(){ // Solo se acpetan numeros enteros?
        Ok(num) => Ok(Datos::Integer(num)),
        Err(_) => Err(ErrorType::InvalidSyntax("Numero invalido en las listas".to_string())),
    }
}

/// Transforma un String o texto en Dato
pub fn operador_to_dato(operador: &Operador) -> Result<Datos, ErrorType> {
    match operador {
        Operador::String(s) => Ok(string_to_number(s.to_string())?),
        Operador::Texto(s) => Ok(Datos::String(s.to_string())),
        _ => Err(ErrorType::InvalidSyntax("Se esperaba una variable".to_string())),
    }
}

/// Extrae el String mas interno de la lista si no tiene mas de un elemento
pub fn extraer_interno_lista(operador: &[Operador]) -> Result<Datos, ErrorType> {
    if operador.len() != 1 {return Err(ErrorType::InvalidSyntax("Cantidad de elemenos incorrecta".to_string()))}
    match &operador[0]{
        Operador::String(s) => Ok(string_to_number(s.to_string())?),
        Operador::Texto(s) => Ok(Datos::String(s.to_string())),
        Operador::Lista(operador) => extraer_interno_lista(operador.as_slice()),
        Operador::Comparador(_) => Err(ErrorType::InvalidSyntax("Comparador inesperado.".to_string()))
    }
}

/// Recibe operador y lo convierte en un Dato
pub fn operador_to_single_dato(operador: &Operador) -> Result<Datos, ErrorType> {
    match &operador {
        Operador::String(_) | Operador::Texto(_) => Ok(operador_to_dato(operador)?),
        Operador::Lista(list) => Ok(extraer_interno_lista(list.as_slice())?),
        Operador::Comparador(_) => Err(ErrorType::InvalidSyntax("Comparador inesperado.".to_string()))
    }
}

/// Extrae el Valor más interno de la lista si no tiene más de un elemento
pub fn extraer_interno_lista_valor(operador: &[Operador]) -> Result<Valor, ErrorType> {
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
pub fn operador_to_single_valor(operador: &Operador) -> Result<Valor, ErrorType> {
    match operador {
        Operador::String(s) => Ok(Valor::String(s.to_string())),
        Operador::Texto(s) => Ok(Valor::Literal(s.to_string())),
        Operador::Lista(list) => extraer_interno_lista_valor(list.as_slice()),
        Operador::Comparador(_) => Err(ErrorType::InvalidSyntax("Comparador inesperado.".to_string())),
    }
}


/// String a OperadorCOmparador
pub fn string_to_comparacion(comparador: &str) -> Result<OperadorComparacion, ErrorType> {
    match comparador{
        "=" => Ok(OperadorComparacion::Igual),
        ">" => Ok(OperadorComparacion::Mayor),
        "<" => Ok(OperadorComparacion::Menor),
        ">=" => Ok(OperadorComparacion::MayorIgual),
        "<=" => Ok(OperadorComparacion::MenorIgual),
        _ => Err(ErrorType::InvalidSyntax("Operador de comparación no válido".to_string()))
    }
}

/// Matchea el tipo de direccion que tiene el ordenamiento
pub fn string_to_direccion(direccion: &str) -> Result<OrderDirection, ErrorType> {
    match direccion {
        "ASC" => Ok(OrderDirection::Asc),
        "DESC" => Ok(OrderDirection::Desc),
        _ => Err(ErrorType::InvalidSyntax("Operador de direccion no válido".to_string()))
    }
}


//
pub fn dato_to_string(dato: &Datos) -> String {
    match dato {
        Datos::Integer(i) => i.to_string(),
        Datos::String(s) => s.to_string(),
    }
}