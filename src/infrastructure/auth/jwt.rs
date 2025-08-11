use chrono::{Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
    errors::Error as JwtError,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const ALGORITHM: Algorithm = Algorithm::HS512;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub jti: String,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Clone)]
pub struct JwtService {
    pub secret: String,
    pub access_exp_minutes: i64,
    pub refresh_exp_days: i64,
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

impl JwtService {
    pub fn new(secret: String, access_exp_minutes: i64, refresh_exp_days: i64) -> Self {
        Self {
            secret,
            access_exp_minutes,
            refresh_exp_days,
        }
    }

    pub fn generate_tokens(&self, user_id: Uuid) -> Tokens {
        let access_token = self.generate_token(user_id, TokenType::Access, self.access_exp_minutes);
        let refresh_token =
            self.generate_token(user_id, TokenType::Refresh, self.refresh_exp_days * 24 * 60);

        Tokens {
            access_token,
            refresh_token,
        }
    }

    fn generate_token(
        &self,
        user_id: Uuid,
        token_type: TokenType,
        expires_in_minutes: i64,
    ) -> String {
        let now = Utc::now();
        let exp = now + Duration::minutes(expires_in_minutes);

        let claims = Claims {
            sub: user_id,
            exp: exp.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            token_type,
        };

        encode(
            &Header::new(ALGORITHM),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .unwrap()
    }

    pub fn validate_token(
        &self,
        token: &str,
        expected_type: TokenType,
    ) -> Result<Claims, JwtError> {
        let mut validation = Validation::new(ALGORITHM);
        validation.validate_exp = true;

        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )?;

        if token_data.claims.token_type != expected_type {
            return Err(JwtError::from(ErrorKind::InvalidToken));
        }

        Ok(token_data.claims)
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims, JwtError> {
        let mut validation = Validation::new(ALGORITHM);
        validation.validate_exp = false; // отключаем проверку срока действия

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_access_token() {
        let jwt = JwtService::new("test-secret".to_string(), 1, 7);
        let user_id = Uuid::new_v4();
        let tokens = jwt.generate_tokens(user_id);

        let claims = jwt
            .validate_token(&tokens.access_token, TokenType::Access)
            .unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.token_type, TokenType::Access);
    }

    #[test]
    fn test_generate_and_validate_refresh_token() {
        let jwt = JwtService::new("test-secret".to_string(), 1, 7);
        let user_id = Uuid::new_v4();
        let tokens = jwt.generate_tokens(user_id);

        let claims = jwt
            .validate_token(&tokens.refresh_token, TokenType::Refresh)
            .unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.token_type, TokenType::Refresh);
    }

    #[test]
    fn test_invalid_token_type_fails() {
        let jwt = JwtService::new("test-secret".to_string(), 1, 7);
        let user_id = Uuid::new_v4();
        let tokens = jwt.generate_tokens(user_id);

        let result = jwt.validate_token(&tokens.access_token, TokenType::Refresh);
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_token() {
        let secret = "test-secret";
        let user_id = Uuid::new_v4();

        // Устанавливаем exp в 100 секунд назад
        let exp = (Utc::now() - Duration::seconds(100)).timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp,
            jti: Uuid::new_v4().to_string(),
            token_type: TokenType::Access,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let jwt = JwtService::new(secret.to_string(), 1, 1);
        let result = jwt.validate_token(&token, TokenType::Access);

        assert!(result.is_err());
    }

    #[test]
    fn test_decode_token_without_validation() {
        let secret = "test-secret";
        let user_id = Uuid::new_v4();
        let exp = (Utc::now() - Duration::minutes(10)).timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp,
            jti: Uuid::new_v4().to_string(),
            token_type: TokenType::Access,
        };

        let token = encode(
            &Header::new(ALGORITHM),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let jwt = JwtService::new(secret.to_string(), 1, 1);
        let decoded = jwt.decode_token(&token).unwrap();

        assert_eq!(decoded.sub, user_id);
        assert_eq!(decoded.token_type, TokenType::Access);
    }
}
