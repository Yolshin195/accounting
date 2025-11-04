use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub telegram_id: Option<String>,
    pub username: String,
    pub password_hash: String,
    pub is_system: bool,
}

#[derive(Debug, Clone)]
pub struct UserAuth {
    pub id: Uuid,
    pub telegram_id: Option<String>,
    pub username: String,
}
