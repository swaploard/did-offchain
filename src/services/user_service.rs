use crate::models::user::User;
use uuid::Uuid;
use chrono::Utc;

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


pub async fn create_user(user: User) -> User { 
    let now = Utc::now();

    User {
        id: Uuid::new_v4(),
        created_at: now,
        updated_at: now,
        last_seen: now,
        is_online: false,
        ..user 
    }
}