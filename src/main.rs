/// invocamos todas las librerias y modulos necesarios
pub mod core;
pub mod models;
use std::env;
use std::fs;
use crate::core::commands;
use crate::core::storage::Storage;
use crate::models::task::Status;
use crate::models::users;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Nos aseguramos que la carpeta data existe
    fs::create_dir_all("data").ok();

    let users_storage = Storage::new("data/users.json".to_string());
    let tasks_storage = Storage::new("data/tasks.json".to_string());

    // Intentamos cargar la sesión actual al iniciar el programa
    let current_user = commands::get_logged_user(&users_storage);

    if args.len() < 2 {
        print_usage(current_user.as_ref());
        return;
    }

    let command = args[1].as_str();

    match command {
        //Comandos Públicos
        "register" => {
            if current_user.is_some() {
                println!("Error: Ya has iniciado sesión. Cierra la sesión primero para registrar un nuevo usuario.");
                return;
            }
            if args.len() > 3 {
                println!("ERROR !. Demasiados argumentos. Si el nombre de usuario tiene espacios, usa comillas (ej: \"Mi Nombre\").");
                return;
            }
            if args.len() < 3 {
                println!("ERROR !. Uso correcto: register \"usuario\"");
                return;
            }
            if let Ok(password) = rpassword::prompt_password("Introduce tu contraseña para registrarte: ") {
                commands::register(&args[2], &password, &users_storage);
            } else {
                println!("Error al leer la contraseña.");
            }
        }
        "login" => {
            if current_user.is_some() {
                println!("Error: Ya has iniciado sesión. Cierra la sesión primero para cambiar de usuario.");
                return;
            }
            if args.len() > 3 {
                println!("ERROR !. Demasiados argumentos. Si el nombre de usuario tiene espacios, usa comillas.");
                return;
            }
            if args.len() < 3 {
                println!("ERROR !. Uso correcto: login \"usuario\"");
                return;
            }
            if let Ok(password) = rpassword::prompt_password("Introduce tu contraseña para iniciar sesión: ") {
                commands::login(&args[2], &password, &users_storage);
            } else {
                println!("Error al leer la contraseña.");
            }
        }
        "logout" => {
            commands::logout();
        }

        //Comandos Privados
        _ => {
            let user = match current_user {
                Some(u) => u,
                None => {
                    println!("Error: Debes iniciar sesión primero con 'cargo run -- login <user> <pass>'");
                    return;
                }
            };

            match command {
                // llamamos al comando para agregar tarea
                "add" => {
                    if args.len() > 4 {
                        println!("ERROR !. Demasiados argumentos. Si el título o la descripción tienen espacios, encuádralos entre comillas (ej: \"Mi Tarea\").");
                        return;
                    }
                    if args.len() < 4 {
                        println!("ERROR !. Uso correcto: add \"título\" \"descripción\"");
                        return;
                    }
                    commands::add_task(args[2].clone(), args[3].clone(), &user, &tasks_storage);
                }
                // llamamos al comando para listar tareas
                "list" => {
                    // El filtro es opcional
                    let filter = if args.len() > 2 {
                        Some(args[2].clone())
                    } else {
                        None
                    };
                    commands::list_tasks(filter, &user, &tasks_storage);
                }
                // llamamos al comando para actualizar tarea
                "update" => {
                    if args.len() > 4 {
                        println!("ERROR !. Demasiados argumentos. Si la nueva descripción tiene espacios, encuádrala entre comillas.");
                        return;
                    }
                    if args.len() < 4 {
                        println!("ERROR !. Uso correcto: update <id> \"nueva_desc\"");
                        return;
                    }
                    if let Ok(id) = args[2].parse::<u32>() {
                        commands::update_task(id, args[3].clone(), &user, &tasks_storage);
                    }
                }
                // llamamos al comando para eliminar tarea
                "delete" => {
                    if args.len() < 3 {
                        println!("ERROR !. Uso correcto: delete <id>");
                        return;
                    }
                    if let Ok(id) = args[2].parse::<u32>() {
                        commands::delete_task(id, &user, &tasks_storage);
                    }
                }
                // llamamos al comando para cambiar estado a en progreso
                "mark-in-progress" => {
                    if args.len() < 3 {
                        println!("ERROR !. Uso correcto: mark-in-progress <id>");
                        return;
                    }
                    if let Ok(id) = args[2].parse::<u32>() {
                        commands::update_task_status(id, Status::InProgress, &user, &tasks_storage);
                    }
                }
                // llamamos al comando para cambiar estado a done
                "mark-done" => {
                    if args.len() < 3 {
                        println!("ERROR !. Uso correcto: mark-done <id>");
                        return;
                    }
                    if let Ok(id) = args[2].parse::<u32>() {
                        commands::update_task_status(id, Status::Done, &user, &tasks_storage);
                    }
                }
                // si el comando no es reconocido, mostramos un mensaje de error
                _ => println!(
                    "Comando '{}' no reconocido. Escribe solo 'cargo run' para ver la ayuda.",
                    command
                ),
            }
        }
    }
}

/// Muestra la ayuda del programa
fn print_usage(user: Option<&users::User>) {
    println!("--- Task Tracker CLI ---");
    if let Some(u) = user {
        println!("Sesión activa: {}", u.username());
    } else {
        println!("Estado: No has iniciado sesión.");
    }
    println!("\nUso:");
    println!("cargo run -- register <usuario>                - Crear una cuenta (pedirá contraseña de forma segura)");
    println!("cargo run -- login <usuario>                   - Iniciar sesión (pedirá contraseña de forma segura)");
    println!("cargo run -- logout                            - Cerrar sesión actual");

    println!("\nAcciones (requieren estar logueado):");
    println!("cargo run -- add \"título\" \"desc\"               - Añadir nueva tarea");
    println!(
        "cargo run -- list [todo|in-progress|done]      - Listar tus tareas (con filtro opcional)"
    );
    println!("cargo run -- update <id> \"nueva_desc\"          - Cambiar descripción de una tarea");
    println!("cargo run -- delete <id>                       - Eliminar una tarea");
    println!("cargo run -- mark-in-progress <id>             - Cambiar estado a En Progreso");
    println!("cargo run -- mark-done <id>                    - Cambiar estado a Terminada");
}
