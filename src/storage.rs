/// traemos el objeto Task
/// libreria para manejar lo relacionado a los Json
/// para leer y escribir archivos
/// para manejar posibles errores
use crate::task::Task;
use serde_json;
use std::fs;
use std::io;

/// creamos la estructura Storage que se encarga de manejar el archivo Json
pub struct Storage {
    file_path: String,
}

impl Storage {
    /// constructor de la estructura Storage
    pub fn new(file_path: String) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}
