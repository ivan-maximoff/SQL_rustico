/// Enum para representar valores que pueden ser una cadena o un literal.
#[derive(Debug, PartialEq)]
pub enum Valor {
    String(String),
    Literal(String),
}
