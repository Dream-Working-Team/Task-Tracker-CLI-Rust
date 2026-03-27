/// invocamos chrono para el manejo de las fechas
/// invocamos serde para el manejo y creacion de los Json
use chrono::Local;
use serde::{Deserialize, Serialize};

/// funciones de serde que entienden la forma del objeto y ayudan a convertirla en Json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// definimos los atributos del objeto
    id: u32,
    user_id: u32,
    title: String,
    description: String,
    status: Status,
    created_date: String,
    update_date: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    /// establecemos un tipo de dato enum para definir los posibles estados de la tarea
    New,
    InProgress,
    Done,
}

impl Task {
    /// Creamos el constructor para el objeto Task
    pub fn new(id: u32, user_id: u32, title: String, description: String) -> Self {
        // invocamos la fecha actual
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            id,
            description,
            title,
            user_id,
            // establecemos el estado inicial de la tarea como New
            status: Status::New,
            // clonamos la fecha actual para la fecha de creacion
            created_date: now.clone(),
            // asignamos la fecha actual para la fecha de actualizacion
            update_date: now,
        }
    }
    /// creamos getters para cada uno de los atributos del objeto Task
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn user_id(&self) -> u32 {
        self.user_id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn update_date(&self) -> &str {
        &self.update_date
    }
    /// creamos un setter para el atributo status
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        // actualizamos la fecha de actualizacion con la fecha del cambio de status
        self.update_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }
    /// creamos un setter para el atributo description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        // actualizamos la fecha de actualizacion con la fecha del cambio de status
        self.update_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }
}
