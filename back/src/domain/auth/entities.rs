#[derive(Clone)]
pub struct AuthEntity {
    pub password: String,
}

impl AuthEntity {
    pub fn new(password: String) -> Self {
        Self { password }
    }
}
