use serde::{Deserialize, Serialize};
use crate::models::user::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_type: String,
    pub role: UserRole,
}
