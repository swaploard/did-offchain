use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;
use sqlx::{Type, FromRow};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Type, ToSchema)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Validate, FromRow)]
pub struct User {
    pub id: Uuid,
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_online: bool,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: UserRole,
}


#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Validate)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub role: Option<UserRole>,
}