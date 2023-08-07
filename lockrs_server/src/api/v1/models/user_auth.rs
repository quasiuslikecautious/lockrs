use uuid::Uuid;

pub struct UserAuthModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

impl UserAuthModel {
    pub fn new(id: &Uuid, email: &str, password_hash: &str) -> Self {
        Self {
            id: id.to_owned(),
            email: email.to_owned(),
            password_hash: password_hash.to_owned(),
        }
    }
}

impl std::fmt::Debug for UserAuthModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserAuthModel {{ {:?}, {:?}, password_hash: ######## }}",
            self.id, self.email
        )
    }
}

pub struct UserLoginCredentials {
    pub email: String,
    pub password: String,
}

impl UserLoginCredentials {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_owned(),
            password: password.to_owned(),
        }
    }
}

impl std::fmt::Debug for UserLoginCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserLoginCredentials: {{ {:?}, password: ******** }}",
            self.email
        )
    }
}

pub struct UserRegistration {
    pub email: String,
    pub password: String,
}

impl UserRegistration {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_owned(),
            password: password.to_owned(),
        }
    }
}

impl std::fmt::Debug for UserRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserRegistration: {{ {:?}, password: ******** }}",
            self.email
        )
    }
}

pub struct UserRegisterModel {
    pub email: String,
    pub password_hash: String,
}

impl UserRegisterModel {
    pub fn new(email: &str, password_hash: &str) -> Self {
        Self {
            email: email.to_owned(),
            password_hash: password_hash.to_owned(),
        }
    }
}

impl std::fmt::Debug for UserRegisterModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserRegistration: {{ {:?}, password_hash: ######## }}",
            self.email
        )
    }
}
