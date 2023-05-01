use actix_identity::Identity;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl User {
    pub fn new(id: i32, username: String, password: String, email: String) -> Self {
        User {
            id,
            username,
            password,
            email,
        }
    }

    pub fn from_identity(identity: &Identity) -> Option<Self> {
        identity.get::<Self>().cloned()
    }

    pub fn to_identity(&self, identity: &Identity) {
        identity.set(self.clone());
    }
}
