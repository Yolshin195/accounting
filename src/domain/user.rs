use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub telegram_id: Option<String>,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct UserAuth {
    pub id: Uuid,
    pub telegram_id: Option<String>,
    pub username: String,
}
