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
    /// funcion para guardar las tareas en el archivo Json
    pub fn save_tasks(&self, tasks: &Vec<Task>) -> io::Result<()> {
        /// convertimos el vector de tareas en un string Json mediante serde
        /// si hay un error lo convertimos a un error de io
        let json = serde_json::to_string_pretty(tasks)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        /// escribimos en el archivo usando la ruta guardada
        fs::write(&self.file_path, json)?;
        /// retornamos Ok(()) para indicar que todo salio bien
        Ok(())
    }
    pub fn read_tasks(&self) -> io::Result<Vec<Task>> {
        /// verificamos la existencia del archivo
        if !std::path::Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }
        /// leemos el archivo
        let archive_content = fs::read_to_string(&self.file_path)?;
        /// usamos serde para convertir el string Json en un vector
        /// si hay un error lo convertimos a un error de io
        let tasks = serde_json::from_str(&archive_content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(tasks)
    }
}
