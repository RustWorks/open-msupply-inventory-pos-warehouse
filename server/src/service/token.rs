use std::sync::RwLock;

use anyhow::anyhow;
use chrono::Utc;
use jsonwebtoken::errors::{Error as JWTError, ErrorKind as JWTErrorKind};
use serde::{Deserialize, Serialize};

use super::token_bucket::TokenBucket;

#[derive(Debug, Serialize, Deserialize)]
enum Audience {
    /// Token is for general api usage
    Api,
    /// Token can be used for a token refresh
    TokenRefresh,
}

// TODO: make the issuer configurable?
const ISSUER: &str = "om-supply-remote-server";

#[derive(Debug, Serialize, Deserialize)]
pub struct OmSupplyClaim {
    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    exp: usize,

    /// Audience
    aud: Audience,
    /// Issued at (as UTC timestamp)
    iat: usize,
    /// Issuer
    iss: String,
    /// Subject (user id the token refers to)
    sub: String,
}

/// Error for getting a JWT token
#[derive(Debug)]
pub enum JWTIssuingError {
    CanNotCreateToken(JWTError),
    ConcurrencyLockError(anyhow::Error),
}

#[derive(Debug)]
pub enum JWTValidationError {
    ExpiredSignature,
    NotAnApiToken,
    InvalidToken(JWTError),
    /// Token has been invalidated on the backend
    TokenInvalided,
    ConcurrencyLockError(anyhow::Error),
}

#[derive(Debug)]
pub enum JWTRefreshError {
    ExpiredSignature,
    NotARefreshToken,
    InvalidToken(JWTError),
    FailedToCreateNewToken(JWTError),
    /// Token has been invalidated on the backend
    TokenInvalided,
    ConcurrencyLockError(anyhow::Error),
}

#[derive(Debug)]
pub enum JWTLogoutError {
    ConcurrencyLockError(anyhow::Error),
}

#[derive(Debug)]
pub struct TokenPair {
    /// The JWT token
    pub token: String,
    /// expiry date of the token
    pub expiry_date: usize,
    /// The JWT refresh token
    pub refresh: String,
    /// Expiry date of the refresh token
    pub refresh_expiry_date: usize,
}

pub struct TokenService<'a> {
    token_bucket: &'a RwLock<TokenBucket>,
    jwt_token_secret: &'a [u8],
}

impl<'a> TokenService<'a> {
    pub fn new(token_bucket: &'a RwLock<TokenBucket>, jwt_token_secret: &'a [u8]) -> Self {
        TokenService {
            token_bucket,
            jwt_token_secret,
        }
    }

    /// Creates new json web token for a given user
    ///
    /// # Arguments
    ///
    /// * `valid_for` - duration (sec) for how long the token will be valid
    /// * `refresh_token_valid_for` - duration (sec) for how long the refresh token will be valid
    pub fn jwt_token(
        &mut self,
        user_id: &str,
        valid_for: usize,
        refresh_token_valid_for: usize,
    ) -> Result<TokenPair, JWTIssuingError> {
        let pair = create_jwt_pair(
            user_id,
            self.jwt_token_secret,
            valid_for,
            refresh_token_valid_for,
        )
        .map_err(|err| JWTIssuingError::CanNotCreateToken(err))?;

        // add tokens to bucket
        let mut token_bucket = self
            .token_bucket
            .write()
            .map_err(|e| JWTIssuingError::ConcurrencyLockError(anyhow!("jwt_token: {}", e)))?;
        token_bucket.put(user_id, &pair.token, pair.expiry_date);
        token_bucket.put(user_id, &pair.refresh, pair.refresh_expiry_date);

        Ok(pair)
    }

    /// Get a new token and also update the refresh token
    ///
    /// # Arguments
    /// * `valid_for` - duration (sec) for how long the token will be valid
    /// * `refresh_token_valid_for` - duration (sec) for how long the refresh token will be valid
    pub fn refresh_token(
        &mut self,
        refresh_token: &str,
        valid_for: usize,
        refresh_token_valid_for: usize,
    ) -> Result<TokenPair, JWTRefreshError> {
        let mut validation = jsonwebtoken::Validation::default();
        validation.set_audience(&vec![format!("{:?}", Audience::TokenRefresh)]);
        validation.iss = Some(ISSUER.to_string());
        let decoded = jsonwebtoken::decode::<OmSupplyClaim>(
            refresh_token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_token_secret),
            &validation,
        )
        .map_err(|err| match err.kind() {
            JWTErrorKind::ExpiredSignature => JWTRefreshError::ExpiredSignature,
            JWTErrorKind::InvalidAudience => JWTRefreshError::NotARefreshToken,
            _ => JWTRefreshError::InvalidToken(err),
        })?;

        let user_id = decoded.claims.sub;
        let pair = create_jwt_pair(
            &user_id,
            self.jwt_token_secret,
            valid_for,
            refresh_token_valid_for,
        )
        .map_err(|err| JWTRefreshError::FailedToCreateNewToken(err))?;

        // Check token is still in the list of valid tokens
        let mut token_bucket = self
            .token_bucket
            .write()
            .map_err(|e| JWTRefreshError::ConcurrencyLockError(anyhow!("refresh_token: {}", e)))?;
        if !token_bucket.contains(&user_id, refresh_token) {
            return Err(JWTRefreshError::TokenInvalided);
        }

        // add new tokens to bucket
        token_bucket.put(&user_id, &pair.token, pair.expiry_date);
        token_bucket.put(&user_id, &pair.refresh, pair.refresh_expiry_date);
        // Shorten the expiry time of the old refresh token.
        //
        // Note, if the client goes offline before receiving the new refresh token the user might
        // need to login again. This might seem random to the user. Lets see if that becomes a real
        // issue.
        let reduced_expiry =
            std::cmp::min(Utc::now().timestamp() as usize + 5 * 60, decoded.claims.exp);
        token_bucket.put(&user_id, refresh_token, reduced_expiry);

        Ok(pair)
    }

    pub fn verify_token(&self, token: &str) -> Result<OmSupplyClaim, JWTValidationError> {
        let mut validation = jsonwebtoken::Validation::default();
        validation.set_audience(&vec![format!("{:?}", Audience::Api)]);
        validation.iss = Some(ISSUER.to_string());
        let decoded = jsonwebtoken::decode::<OmSupplyClaim>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_token_secret),
            &validation,
        )
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                JWTValidationError::ExpiredSignature
            }
            jsonwebtoken::errors::ErrorKind::InvalidAudience => JWTValidationError::NotAnApiToken,
            _ => JWTValidationError::InvalidToken(err),
        })?;

        // Check token is still in the list of valid tokens
        let token_bucket = self.token_bucket.read().map_err(|e| {
            JWTValidationError::ConcurrencyLockError(anyhow!("verify_token: {}", e))
        })?;
        if !token_bucket.contains(&decoded.claims.sub, token) {
            return Err(JWTValidationError::TokenInvalided);
        }
        Ok(decoded.claims)
    }

    /// Log a user out of all sessions
    pub fn logout(&mut self, user_id: &str) -> Result<(), JWTLogoutError> {
        let mut token_bucket = self
            .token_bucket
            .write()
            .map_err(|e| JWTLogoutError::ConcurrencyLockError(anyhow!("logout: {}", e)))?;
        token_bucket.clear(user_id);
        Ok(())
    }
}

/// Creates a token and refresh token pair
fn create_jwt_pair(
    user_id: &str,
    jwt_token_secret: &[u8],
    valid_for: usize,
    refresh_valid_for: usize,
) -> Result<TokenPair, JWTError> {
    let now = Utc::now().timestamp() as usize;
    let expiry_date = now + valid_for;
    let refresh_expiry_date = now + refresh_valid_for;

    // api token
    let api_claims = OmSupplyClaim {
        exp: expiry_date,
        aud: Audience::Api,
        iat: now,
        iss: ISSUER.to_string(),
        sub: user_id.to_owned(),
    };
    let api_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &api_claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_token_secret),
    )?;

    // refresh token
    let refresh_claims = OmSupplyClaim {
        exp: refresh_expiry_date,
        aud: Audience::TokenRefresh,
        iat: now,
        iss: ISSUER.to_string(),
        sub: user_id.to_owned(),
    };
    let refresh_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &refresh_claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_token_secret),
    )?;

    Ok(TokenPair {
        token: api_token,
        expiry_date,
        refresh: refresh_token,
        refresh_expiry_date,
    })
}

#[cfg(test)]
mod user_account_test {
    use crate::service::token_bucket::TokenBucket;

    use super::*;

    #[actix_rt::test]
    async fn test_user_auth() {
        let bucket = RwLock::new(TokenBucket::new());
        const JWT_TOKEN_SECRET: &[u8] = "some secret".as_bytes();
        let user_id = "test_user_id";
        let mut service = TokenService::new(&bucket, JWT_TOKEN_SECRET);

        // should be able to create a new token
        let token_pair = service.jwt_token(user_id, 60, 120).unwrap();

        // should be able to verify token
        let claims = service.verify_token(&token_pair.token).unwrap();
        assert_eq!(user_id, claims.sub);

        // should fail to verify with refresh token
        let err = service.verify_token(&token_pair.refresh).unwrap_err();
        assert!(matches!(err, JWTValidationError::NotAnApiToken));

        // should fail to refresh token refresh with api token
        let err = service
            .refresh_token(&token_pair.token, 60, 120)
            .unwrap_err();
        assert!(matches!(err, JWTRefreshError::NotARefreshToken));

        // should succeed to refresh token
        let token_pair = service.refresh_token(&token_pair.refresh, 60, 120).unwrap();
        let claims = service.verify_token(&token_pair.token).unwrap();
        // important: sub must still match the user id:
        assert_eq!(user_id, claims.sub);

        // should fail to verify and refresh when logged out
        service.logout(&user_id).unwrap();
        let err = service.verify_token(&token_pair.token).unwrap_err();
        assert!(matches!(err, JWTValidationError::TokenInvalided));
        let err = service
            .refresh_token(&token_pair.refresh, 60, 120)
            .unwrap_err();
        assert!(matches!(err, JWTRefreshError::TokenInvalided));
    }

    #[actix_rt::test]
    async fn test_user_auth_token_expiry() {
        let bucket = RwLock::new(TokenBucket::new());
        const JWT_TOKEN_SECRET: &[u8] = "some secret".as_bytes();
        let user_id = "test_user_id";
        let mut service = TokenService::new(&bucket, JWT_TOKEN_SECRET);

        // should be able to create a new token
        let token_pair = service.jwt_token(user_id, 1, 1).unwrap();
        // should be able to verify token
        let claims = service.verify_token(&token_pair.token).unwrap();
        assert_eq!(user_id, claims.sub);

        // granularity is 1 sec so need to wait 2 sec
        std::thread::sleep(std::time::Duration::from_millis(2000));
        let err = service.verify_token(&token_pair.token).unwrap_err();
        assert!(matches!(err, JWTValidationError::ExpiredSignature));
        let err = service
            .refresh_token(&token_pair.refresh, 1, 1)
            .unwrap_err();
        assert!(matches!(err, JWTRefreshError::ExpiredSignature));
    }
}
