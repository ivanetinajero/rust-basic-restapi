use axum::{Json, extract::{Path, Query, State}, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, sqlx::types::chrono};

use crate::{context::AppContext, users::dto::{CreateUserParams, Pagination, UserCreated}};

pub async fn read_user(Path(user_id): Path<u32>) -> impl IntoResponse {
    println!("Received request for user with ID: {}", user_id);
    format!("User ID: {}", user_id)
}

pub async fn read_users(Query(pagination): Query<Pagination>) -> impl IntoResponse {
    println!("fetching users on page  {} with page size {}", pagination.page, pagination.page_size.unwrap_or(10));
    format!("fetching users on page {} with page size {}", pagination.page, pagination.page_size.unwrap_or(10))
}

pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateUserParams>
) -> impl IntoResponse {
    
    println!("Creating user with name: {} and username: {}", payload.name, payload.username);
    let created_at = chrono::Utc::now().naive_utc();

    let model = schemas::user::ActiveModel{
        id: ActiveValue::NotSet,
        username: ActiveValue::Set(payload.username),
        password: ActiveValue::Set("default_password".to_string()), // This should be replaced with actual password handling logic
        disabled: ActiveValue::Set(0), // Assuming new users are enabled by default
        created_at: ActiveValue::Set(created_at),
        creator_id: ActiveValue::Set(1), // This should be replaced with actual creator ID logic
    }.insert(&ctx.conn).await.expect("Failed to create user");
           
    let user = UserCreated {
        id: 1
    };
    Json(user)
}