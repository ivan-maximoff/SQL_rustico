use std::collections::HashMap;

use crate::{dato::Datos, error::ErrorType};

use super::evaluar::Evaluar;

#[derive(Debug, PartialEq)]
pub enum ExpresionBooleana {
    Comparacion {
        izq: String,
        operador: OperadorComparacion,
        der: String
    },
    And(Box<ExpresionBooleana>, Box<ExpresionBooleana>),
    Or(Box<ExpresionBooleana>, Box<ExpresionBooleana>),
    Not(Box<ExpresionBooleana>),   
}

#[derive(Debug, PartialEq)]
pub enum OperadorComparacion {
    Igual,
    Menor,
    Mayor,
}

impl Evaluar for ExpresionBooleana {
    fn evaluar(&self, fila: &HashMap<String, Datos>) -> Result<bool, ErrorType> {
        match self {
            ExpresionBooleana::Comparacion { izq, operador, der } => {
                if !fila.contains_key(izq)

                match operador {
                    OperadorComparacion::Igual => Ok(valor_izquierdo == valor_derecho),
                    OperadorComparacion::Menor => Ok(valor_izquierdo < valor_derecho),
                    OperadorComparacion::Mayor => Ok(valor_izquierdo > valor_derecho),
                }
                Ok(true)
            },
            ExpresionBooleana::And(expr1, expr2) => {
                Ok(expr1.evaluar(fila)? && expr2.evaluar(fila)?)
            },
            ExpresionBooleana::Or(expr1, expr2) => {
                Ok(expr1.evaluar(fila)? || expr2.evaluar(fila)?)
            },
            ExpresionBooleana::Not(expr) => {
                Ok(!expr.evaluar(fila)?)
            },
        }
    }
}