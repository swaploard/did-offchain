use crate::errors::user_errors::UserServiceError;
use crate::models::auth::SignupRequest;
use crate::models::user::User;
use crate::utils::password;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn register_user(
    db: &PgPool,
    payload: SignupRequest,
) -> Result<String, UserServiceError> {
    let role = "User".to_string(); // Default role
    let existing: Option<(Uuid,)> =
        sqlx::query_as("SELECT id FROM users WHERE username = $1 OR email = $2")
            .bind(&payload.username)
            .bind(&payload.email)
            .fetch_optional(db)
            .await?;

    if existing.is_some() {
        return Err(UserServiceError::UserExists);
    }

    let password_hash =
        password::hash_password(&payload.password).map_err(|_e| UserServiceError::HashError)?;

    let rec: User = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (
            username, 
            email, 
            password_hash, 
            display_name, 
            avatar_url, 
            role
        ) VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING 
            id,
            username,
            email,
            password_hash,
            display_name,
            avatar_url,
            is_online,
            last_seen,
            created_at,
            updated_at,
            role
        "#,
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.display_name)
    .bind(&payload.avatar_url)
    .bind(&role)
    .fetch_one(db)
    .await?;

    Ok(rec.id.to_string())
}

pub async fn authenticate_user(
    db: &PgPool,
    username: &str,
    password: &str,
) -> Result<User, UserServiceError> {
    let maybe_user: Option<User> = sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            display_name,
            avatar_url,
            is_online,
            last_seen,
            created_at,
            updated_at,
            role
        FROM users
        WHERE username = $1
        "#,
    )
    .bind(username)
    .fetch_optional(db)
    .await?;

    let user = maybe_user.ok_or(UserServiceError::BadCredentials)?;

    password::verify_password(password, &user.password_hash)
        .map_err(|_| UserServiceError::BadCredentials)?;

    Ok(user)
}
