
/// Enum que representa diferentes tipos de operadores en una consulta SQL.
/// Contiene variantes para cadenas de texto, listas de operadores, textos literales y comparadores.
#[derive(Debug, PartialEq)]
pub enum Operador {
    String(String),
    Lista(Vec<Operador>),
    Texto(String),
    Comparador(String)
}