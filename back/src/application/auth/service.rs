use crate::domain::auth::AuthEntity;
use sha2::{Digest, Sha256};

pub struct AuthService {
    pub auth_entity: AuthEntity,
}

impl AuthService {
    pub fn new(auth_entity: AuthEntity) -> Self {
        Self { auth_entity }
    }

    pub fn authenticate(&self, hash: &str) -> bool {
        let stored_hash = hash_password(&self.auth_entity.password);
        hash.eq_ignore_ascii_case(&stored_hash)
    }
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|byte| format!("{:02x}", byte)).collect()
}
