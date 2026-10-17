use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use domain_core::AggregateRoot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    email: String,
    password_hash: String,
    is_active: bool,
    customer_id: Option<Uuid>,
}

impl AggregateRoot for User {
    type Id = Uuid;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl User {
    /// Registers a new user with an Argon2 hashed password
    pub fn register(
        email: String,
        raw_password: &str,
        customer_id: Option<Uuid>,
    ) -> Result<Self, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();

        Ok(Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            is_active: true,
            customer_id,
        })
    }

    pub fn verify_password(&self, raw_password: &str) -> bool {
        let parsed_hash = match PasswordHash::new(&self.password_hash) {
            Ok(h) => h,
            Err(_) => return false,
        };
        Argon2::default()
            .verify_password(raw_password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn customer_id(&self) -> Option<Uuid> {
        self.customer_id
    }
}
