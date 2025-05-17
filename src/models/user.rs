use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, utoipa::ToSchema)]
pub struct User {
    pub id: i32,
    pub name: String,
}
