use crate::models::user::User;

pub async fn fetch_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Alice".into() },
        User { id: 2, name: "Bob".into() },
    ]
}
