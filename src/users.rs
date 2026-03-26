/// invocamos bcrypt para el manejo de las contraseñas
/// invocamos serde para el manejo y creacion de los Json
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// creamos la estructura User que se encarga de manejar los usuarios
pub struct User {
    // definimos atributos
    id: u32,
    username: String,
    password_hash: String,
}

impl User {
    // constructor del objeto User
    pub fn new(id: u32, username: String, password: &str) -> Result<Self, bcrypt::BcryptError> {
        let password_hash = hash(password, DEFAULT_COST)?;
        Ok(Self {
            id,
            username,
            password_hash,
        })
    }

    // Getters
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}
