use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use anyhow::{anyhow, Result};

/// Хеширует пароль с уникальной солью и возвращает PHC string
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| anyhow!("Failed to hash password: {:?}", e))

}

/// Проверяет пароль по PHC string, возвращает true если совпадает
pub fn verify_password(hash: &str, password: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow!("Invalid password hash: {:?}", e))?;

    let argon2 = Argon2::default();
    let result = argon2.verify_password(password.as_bytes(), &parsed_hash);

    Ok(result.is_ok())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify_success() {
        let password = "hunter42";

        let hash = hash_password(password).expect("Hashing should succeed");
        assert!(verify_password(&hash, password).expect("Verification should succeed"));
    }

    #[test]
    fn test_password_verify_fail_on_wrong_password() {
        let password = "hunter42";
        let wrong_password = "hunter43";

        let hash = hash_password(password).expect("Hashing should succeed");
        assert!(!verify_password(&hash, wrong_password).expect("Verification should succeed"));
    }

    #[test]
    fn test_password_verify_fail_on_invalid_hash() {
        let password = "hunter42";
        let invalid_hash = "not-a-valid-phc-hash";

        let result = verify_password(invalid_hash, password);
        assert!(result.is_err(), "Expected error for invalid hash string");
    }
}