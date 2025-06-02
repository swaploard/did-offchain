use crate::models::user::{User, CreateUserRequest};
use uuid::Uuid;
use chrono::Utc;
use sqlx::PgPool;

pub async fn fetch_users() -> Vec<User> {
    let now = Utc::now();
    
    vec![
        User {
            id: Uuid::new_v4(),
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            display_name: Some("Alice A.".to_string()),
            avatar_url: Some("https://example.com/avatar1.png".to_string()),
            is_online: true,
            last_seen: now,
            created_at: now,
            updated_at: now,
        },
        User {
            id: Uuid::new_v4(),
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            display_name: Some("Bob B.".to_string()),
            avatar_url: Some("https://example.com/avatar2.png".to_string()),
            is_online: false,
            last_seen: now,
            created_at: now,
            updated_at: now,
        },
    ]
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