use crate::storage::Storage;
use crate::task::{Status, Task};
use crate::users::User;
use std::fs;

/// Función para autenticar a un usuario (uso interno)
fn authenticate(username: &str, password: &str, storage: &Storage) -> Option<User> {
    let users = storage.read_data::<User>().unwrap_or_else(|_| Vec::new());
    users
        .into_iter()
        .find(|u| u.username() == username && u.verify_password(password))
}

/// Inicia sesión guardando el nombre del usuario en un archivo temporal
pub fn login(username: &str, password: &str, storage: &Storage) {
    if let Some(_) = authenticate(username, password, storage) {
        if let Err(e) = fs::write(".session", username) {
            println!("Error al crear la sesión: {}", e);
        } else {
            println!("¡Inicio de sesión con éxito! Bienvenido, {}.", username);
        }
    } else {
        println!("Error: Usuario o contraseña incorrectos.");
    }
}

/// Cierra la sesión borrando el archivo temporal
pub fn logout() {
    if fs::remove_file(".session").is_ok() {
        println!("Sesión cerrada correctamente.");
    } else {
        println!("No había ninguna sesión activa.");
    }
}

/// Obtiene el usuario actualmente logueado
pub fn get_logged_user(storage: &Storage) -> Option<User> {
    if let Ok(username) = fs::read_to_string(".session") {
        let users = storage.read_data::<User>().unwrap_or_else(|_| Vec::new());
        users.into_iter().find(|u| u.username() == username)
    } else {
        None
    }
}

/// Función para registrar un nuevo usuario
pub fn register(username: &str, password: &str, storage: &Storage) {
    let mut users = storage.read_data::<User>().unwrap_or_else(|_| Vec::new());

    if users.iter().any(|u| u.username() == username) {
        println!("Error: El usuario ya existe.");
        return;
    }

    let new_id = (users.len() as u32) + 1;
    match User::new(new_id, username.to_string(), password) {
        Ok(new_user) => {
            users.push(new_user);
            if let Err(e) = storage.save_data(&users) {
                println!("Error al guardar usuario: {}", e);
            } else {
                println!("Usuario registrado con éxito: {}", username);
            }
        }
        Err(e) => println!("Error al crear usuario: {}", e),
    }
}

/// Función para añadir una nueva tarea
pub fn add_task(title: String, desc: String, user: &User, storage: &Storage) {
    let mut all_tasks = storage.read_data::<Task>().unwrap_or_else(|_| Vec::new());
    let new_id = (all_tasks.len() as u32) + 1;
    let new_task = Task::new(new_id, user.id(), title, desc);
    all_tasks.push(new_task);
    if let Err(e) = storage.save_data(&all_tasks) {
        println!("Error al guardar tarea: {}", e);
    } else {
        println!("Tarea añadida con éxito.");
    }
}

/// Función para listar las tareas de un usuario con filtro opcional de estado
pub fn list_tasks(status_filter: Option<String>, user: &User, storage: &Storage) {
    let all_tasks = storage.read_data::<Task>().unwrap_or_else(|_| Vec::new());

    // Filtramos primero por el usuario logueado
    let mut tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.user_id() == user.id())
        .collect();

    // Si hay un filtro de estado
    if let Some(status_str) = status_filter {
        tasks.retain(|t| {
            match status_str.as_str() {
                "todo" => t.status() == &Status::New,
                "in-progress" => t.status() == &Status::InProgress,
                "done" => t.status() == &Status::Done,
                _ => true, // Si el filtro no existe, mostramos todo
            }
        });
    }

    if tasks.is_empty() {
        println!("No se encontraron tareas con ese criterio.");
    } else {
        for t in tasks {
            println!(
                "[{}] {} - {:?} (Último cambio: {})",
                t.id(),
                t.title(),
                t.status(),
                t.update_date()
            );
        }
    }
}

/// Función para actualizar la descripción de una tarea
pub fn update_task(id: u32, new_desc: String, user: &User, storage: &Storage) {
    let mut all_tasks = storage.read_data::<Task>().unwrap_or_else(|_| Vec::new());
    if let Some(task) = all_tasks
        .iter_mut()
        .find(|t| t.id() == id && t.user_id() == user.id())
    {
        task.set_description(new_desc);
        storage
            .save_data(&all_tasks)
            .expect("Error al guardar tareas");
        println!("Tarea actualizada con éxito.");
    } else {
        println!("Error: Tarea no encontrada o no te pertenece.");
    }
}

/// Función para eliminar una tarea
pub fn delete_task(id: u32, user: &User, storage: &Storage) {
    let mut all_tasks = storage.read_data::<Task>().unwrap_or_else(|_| Vec::new());
    let original_len = all_tasks.len();
    all_tasks.retain(|t| !(t.id() == id && t.user_id() == user.id()));

    if all_tasks.len() < original_len {
        storage
            .save_data(&all_tasks)
            .expect("Error al guardar tareas");
        println!("Tarea eliminada con éxito.");
    } else {
        println!("Error: Tarea no encontrada o no te pertenece.");
    }
}

/// Función para actualizar el estado de una tarea (helper interno del módulo)
pub fn update_task_status(id: u32, new_status: Status, user: &User, storage: &Storage) {
    let mut all_tasks = storage.read_data::<Task>().unwrap_or_else(|_| Vec::new());
    if let Some(task) = all_tasks
        .iter_mut()
        .find(|t| t.id() == id && t.user_id() == user.id())
    {
        task.set_status(new_status);
        storage
            .save_data(&all_tasks)
            .expect("Error al guardar tareas");
        println!("Estado de la tarea actualizado.");
    } else {
        println!("Error: Tarea no encontrada.");
    }
}
