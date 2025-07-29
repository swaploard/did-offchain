use crate::models::claims::Claims;
use crate::settings::jwt::JWT_CONFIG;
use crate::utils::jwt::decode_token;
use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};

#[derive(Debug, Clone)]
pub struct AuthGuard {
    pub claims: Claims,
}

impl FromRequest for AuthGuard {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(h) => h.to_str().unwrap_or("").to_string(),
            None => return ready(Err(ErrorUnauthorized("Authorization header missing"))),
        };

        if !auth_header.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("Expected Bearer token")));
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();

        match decode_token(token, &JWT_CONFIG.access_secret) {
            Ok(data) => ready(Ok(AuthGuard {
                claims: data.claims,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Invalid or expired token"))),
        }
    }
}

impl AuthGuard {
    pub fn require_role(&self, required: crate::models::user::UserRole) -> Result<(), Error> {
        if self.claims.role == required {
            Ok(())
        } else {
            Err(ErrorUnauthorized("Forbidden: insufficient permissions"))
        }
    }

    pub fn require_any_role(&self, roles: &[crate::models::user::UserRole]) -> Result<(), Error> {
        if roles.contains(&self.claims.role) {
            Ok(())
        } else {
            Err(ErrorUnauthorized("Forbidden: insufficient permissions"))
        }
    }
}
