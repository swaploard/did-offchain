use crate::models::user::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_type: String,
    pub role: UserRole,
}
