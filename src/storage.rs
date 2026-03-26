use serde::de::DeserializeOwned;
/// traemos el objeto Task
/// libreria para manejar lo relacionado a los Json
/// para leer y escribir archivos
/// para manejar posibles errores
use serde::Serialize;
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
    /// funcion para guardar datos en un archivo Json
    pub fn save_data<T: Serialize>(&self, data: &Vec<T>) -> io::Result<()> {
        // convertimos el vector a un string Json mediante serde
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        // escribimos en el archivo usando la ruta guardada
        fs::write(&self.file_path, json)?;
        Ok(())
    }
    pub fn read_data<T: DeserializeOwned>(&self) -> io::Result<Vec<T>> {
        // verificamos la existencia del archivo
        if !std::path::Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }
        // leemos el archivo
        let archive_content = fs::read_to_string(&self.file_path)?;
        // usamos serde para convertir el string Json en un vector
        let data = serde_json::from_str(&archive_content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(data)
    }
}
