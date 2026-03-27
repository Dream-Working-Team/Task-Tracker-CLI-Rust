mod commands;
mod storage;
mod task;
mod users;

use std::env;
use storage::Storage;
use task::Status;

fn main() {
    let args: Vec<String> = env::args().collect();

    let users_storage = Storage::new("users.json".to_string());
    let tasks_storage = Storage::new("tasks.json".to_string());

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
            if args.len() < 4 {
                println!("Uso: register <usuario> <password>");
                return;
            }
            commands::register(&args[2], &args[3], &users_storage);
        }
        "login" => {
            if args.len() < 4 {
                println!("Uso: login <usuario> <password>");
                return;
            }
            commands::login(&args[2], &args[3], &users_storage);
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
                    if args.len() < 4 {
                        println!("Uso: add <título> <descripción>");
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
                    if args.len() < 4 {
                        println!("Uso: update <id> <nueva_desc>");
                        return;
                    }
                    if let Ok(id) = args[2].parse::<u32>() {
                        commands::update_task(id, args[3].clone(), &user, &tasks_storage);
                    }
                }
                // llamamos al comando para eliminar tarea
                "delete" => {
                    if args.len() < 3 {
                        println!("Uso: delete <id>");
                        return;
                    }
                    if let Ok(id) = args[2].parse::<u32>() {
                        commands::delete_task(id, &user, &tasks_storage);
                    }
                }
                // llamamos al comando para cambiar estado a en progreso
                "mark-in-progress" => {
                    if args.len() < 3 {
                        println!("Uso: mark-in-progress <id>");
                        return;
                    }
                    if let Ok(id) = args[2].parse::<u32>() {
                        commands::update_task_status(id, Status::InProgress, &user, &tasks_storage);
                    }
                }
                // llamamos al comando para cambiar estado a done
                "mark-done" => {
                    if args.len() < 3 {
                        println!("Uso: mark-done <id>");
                        return;
                    }
                    if let Ok(id) = args[2].parse::<u32>() {
                        commands::update_task_status(id, Status::Done, &user, &tasks_storage);
                    }
                }
                // si el comando no es reconocido, mostramos un mensaje de error
                _ => println!(
                    "Comando '{}' no reconocido. Escribe solo el programa para ver la ayuda.",
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
    println!("cargo run -- register <usuario> <password>     - Crear una cuenta");
    println!("cargo run -- login <usuario> <password>        - Iniciar sesión persistentemente");
    println!("cargo run -- logout                            - Cerrar sesión actual");

    println!("\nAcciones (requieren estar logueado):");
    println!("cargo run -- add <título> <desc>               - Añadir nueva tarea");
    println!(
        "cargo run -- list [todo|in-progress|done]      - Listar tus tareas (con filtro opcional)"
    );
    println!("cargo run -- update <id> <nueva_desc>          - Cambiar descripción de una tarea");
    println!("cargo run -- delete <id>                       - Eliminar una tarea");
    println!("cargo run -- mark-in-progress <id>             - Cambiar estado a En Progreso");
    println!("cargo run -- mark-done <id>                    - Cambiar estado a Terminada");
}
