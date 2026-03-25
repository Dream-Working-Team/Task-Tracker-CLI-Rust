pub struct Task {
    /// definimos los atributos del objeto
    id: u32,
    title: String,
    description: String,
    status: Status,
    created_date: String,
    update_date: String,
}

pub enum Status {
    /// establecemos un tipo de dato enum para definir los posibles estados de la tarea
    New,
    InPogress,
    Done,
}
