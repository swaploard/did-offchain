use crate::models::user::{User, CreateUserRequest};
use uuid::Uuid;
use chrono::Utc;
use sqlx::PgPool;
use std::error::Error;

pub async fn fetch_users(
    pool: &PgPool
) ->  Result<Vec<User>, Box<dyn Error>> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT *
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}


pub async fn create_user(
    pool: &PgPool,
    user_req: CreateUserRequest,
) -> Result<User, Box<dyn std::error::Error>> {

    let id = Uuid::new_v4();
    let now = Utc::now();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (
            id,
            username,
            email,
            password_hash,
            display_name,
            avatar_url,
            is_online,
            last_seen,
            created_at,
            updated_at
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
        )
        RETURNING *
        "#
    )
    .bind(id)
    .bind(&user_req.username)
    .bind(&user_req.email)
    .bind(&user_req.password_hash)
    .bind(&user_req.display_name)
    .bind(&user_req.avatar_url)
    .bind(false) 
    .bind(now)   
    .bind(now)   
    .bind(now)   
    .fetch_one(pool)
    .await?;

    Ok(user)
}