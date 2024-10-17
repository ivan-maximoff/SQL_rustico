use super::{operador_comparacion::OperadorComparacion, valor::Valor};

/// Enum para representar diferentes tipos de expresiones booleanas.
#[derive(Debug, PartialEq)]
pub enum ExpresionBooleana {
    Comparacion {
        izq: Valor,
        operador: OperadorComparacion,
        der: Valor,
    },
    And(Box<ExpresionBooleana>, Box<ExpresionBooleana>),
    Or(Box<ExpresionBooleana>, Box<ExpresionBooleana>),
    Not(Box<ExpresionBooleana>),
}
