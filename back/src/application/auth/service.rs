use crate::domain::auth::AuthEntity;

pub struct AuthService {
    pub auth_entity: AuthEntity,
}

impl AuthService {
    pub fn new(auth_entity: AuthEntity) -> Self {
        Self { auth_entity }
    }

    pub fn authenticate(&self, password: &str) -> bool {
        self.auth_entity.password == password
    }
}
