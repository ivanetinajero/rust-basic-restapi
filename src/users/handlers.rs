use axum::{Json, extract::{Path, Query}, response::IntoResponse};

use crate::users::dto::{CreateUserParams, Pagination, UserCreated};

pub async fn read_user(Path(user_id): Path<u32>) -> impl IntoResponse {
    println!("Received request for user with ID: {}", user_id);
    format!("User ID: {}", user_id)
}

pub async fn read_users(Query(pagination): Query<Pagination>) -> impl IntoResponse {
    println!("fetching users on page  {} with page size {}", pagination.page, pagination.page_size.unwrap_or(10));
    format!("fetching users on page {} with page size {}", pagination.page, pagination.page_size.unwrap_or(10))
}

pub async fn create_user(Json(payload): Json<CreateUserParams>) -> impl IntoResponse {
    println!("Creating user with name: {} and username: {}", payload.name, payload.username);
    let user = UserCreated {
        id: 1, // This should be replaced with actual ID generation logic
        name: payload.name,
        username: payload.username,
    };
    Json(user)
}