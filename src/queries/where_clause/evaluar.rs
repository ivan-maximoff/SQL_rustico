use std::collections::HashMap;

use crate::{dato::Datos, errores::error::ErrorType};

/// Trait para evaluar expresiones booleanas en función de una fila de datos.
pub trait Evaluar {
    fn evaluar(&self, fila: &HashMap<String, Datos>) -> Result<bool, ErrorType>;
}
