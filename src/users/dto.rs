use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: Option<u32>
}

#[derive(Deserialize)]
pub struct CreateUserParams {
    pub name: String,
    pub username: String
}

#[derive(Serialize)]
pub struct UserCreated {
    pub id: u32,
    pub name: String,
    pub username: String
}