use std::collections::HashMap;

use crate::{dato::Datos, error::ErrorType};

pub trait Evaluar {
    fn evaluar(&self, fila: &HashMap<String, Datos>) -> Result<bool, ErrorType>;
}