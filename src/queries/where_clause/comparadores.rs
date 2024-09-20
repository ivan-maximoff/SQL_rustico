use std::collections::HashMap;

use crate::{dato::Datos, errores::error::ErrorType};

use super::{evaluar::Evaluar, expresion_booleana::ExpresionBooleana, operador_comparacion::OperadorComparacion, valor::Valor};

/// Evalúa el valor de una expresión booleano.
fn evaluar_valor(valor: &Valor, fila: &HashMap<String, Datos>) -> Result<Datos, ErrorType> {
    match valor {
        Valor::String(s) => {
            if let Some(dato) = fila.get(s) {
                match dato {
                    Datos::Integer(s) => Ok(Datos::Integer(*s)),
                    Datos::String(s) => Ok(Datos::String(s.to_string()))
                } 
            } else if let Ok(num) = s.parse::<i64>() {
                Ok(Datos::Integer(num))
            } else {
                Err(ErrorType::InvalidSyntax(format!("El valor '{}' no existe como columna ni es un numero.", s)))
            }
        },
        Valor::Literal(lit) => Ok(Datos::String(lit.to_string()))
    }
}

impl Evaluar for ExpresionBooleana {
    /// Evalúa una expresión booleana utilizando los datos proporcionados.
    fn evaluar(&self, fila: &HashMap<String, Datos>) -> Result<bool, ErrorType> {
        match self {
            ExpresionBooleana::Comparacion { izq, operador, der } => {
                let valor_izq = evaluar_valor(izq, fila)?;
                let valor_der = evaluar_valor(der, fila)?;
                match operador {
                    OperadorComparacion::Igual => Ok(valor_izq == valor_der),
                    OperadorComparacion::Menor => Ok(valor_izq < valor_der),
                    OperadorComparacion::Mayor => Ok(valor_izq > valor_der),
                    OperadorComparacion::MenorIgual => Ok(valor_izq <= valor_der),
                    OperadorComparacion::MayorIgual => Ok(valor_izq >= valor_der),
                }
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