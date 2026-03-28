# Task Tracker CLI (Rust)

Un gestor de tareas de interfaz de línea de comandos (CLI) simple, seguro y ultrarrápido construido completamente en Rust. Este proyecto permite realizar un seguimiento de tus tareas diarias, gestionando sus estados (por hacer, en progreso, completadas) mediante almacenamiento persistente en un archivo JSON local.

## 🚀 Características

* **Dependencias:** Construido con la biblioteca estándar de Rust (`std::env`, `std::fs`, `std::io`) y utilizando dependencias externas como Serde, Chrono y Rpassword.
* **Almacenamiento Local:** Las tareas se guardan automáticamente en un archivo `tasks.json` en el directorio actual. El archivo se inicializa de forma segura si no existe.
* **Gestión Completa (CRUD):** Permite añadir, actualizar y eliminar tareas fácilmente desde la terminal.
* **Control de Estados:** Transiciones fluidas entre estados (`todo`, `in-progress`, `done`).
* **Filtros de Búsqueda:** Listado de tareas global o filtrado por su estado actual.
* **Manejo de Errores Robusto:** Aprovecha el potente sistema de tipos de Rust (`Result` y `Option`) para gestionar comandos inválidos, IDs inexistentes o problemas de permisos del sistema de archivos, garantizando una ejecución segura sin pánicos inesperados (*panic-free*).

## 🛠️ Instalación y Compilación

Asegúrate de tener [Rust y Cargo instalados](https://www.rust-lang.org/tools/install) en tu sistema.

1. Clona este repositorio:
   ```bash
   git clone [https://github.com/Dream-Working-Team/Task-Tracker-CLI-Rust.git](https://github.com/Dream-Working-Team/Task-Tracker-CLI-Rust.git)
   cd Task-Tracker-CLI-Rust