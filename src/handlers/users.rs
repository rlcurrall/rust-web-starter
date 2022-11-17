use actix_web::{web::Data, Result};
use paperclip::actix::{
    api_v2_operation, delete, get, post, put,
    web::{Json, Path, Query},
    Apiv2Schema, NoContent,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::DbPool,
    middleware::UserClaims,
    repositories::users::{self, NewUser, UpdateUser, User},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Apiv2Schema)]
pub struct UsersQuery {
    /// The page to return
    page: Option<i64>,
    /// The number of records to return at one time
    per_page: Option<i64>,

    /// Filter users by active status
    active: Option<bool>,

    /// Get users older than the given value
    #[serde(rename = "age[gt]")]
    age_gt: Option<i32>,

    /// Get users younger than the given value
    #[serde(rename = "age[lt]")]
    age_lt: Option<i32>,
}

impl Into<users::UsersQuery> for UsersQuery {
    fn into(self) -> users::UsersQuery {
        users::UsersQuery {
            active: self.active,
            age_gt: self.age_gt,
            age_lt: self.age_lt,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Apiv2Schema)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub age: i32,
    pub active: bool,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        UserResponse {
            id: u.id,
            email: u.email,
            name: u.name,
            age: u.age,
            active: u.active,
        }
    }
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct PaginatedUsers {
    pub total: i64,
    pub members: Vec<UserResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Apiv2Schema)]
pub struct NewUserRequest {
    pub email: String,
    pub name: String,
    pub age: i32,
    pub active: bool,
}

impl Into<NewUser> for NewUserRequest {
    fn into(self) -> NewUser {
        NewUser {
            email: self.email,
            name: self.name,
            age: self.age,
            active: self.active,
        }
    }
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct UpdateUserRequest {
    pub email: String,
    pub name: String,
    pub age: i32,
    pub active: bool,
}

impl Into<UpdateUser> for UpdateUserRequest {
    fn into(self) -> UpdateUser {
        UpdateUser {
            email: self.email,
            name: self.name,
            age: self.age,
            active: self.active,
        }
    }
}

#[api_v2_operation(tags("User"))]
#[get("/users/{id}")]
pub fn get_user(id: Path<i32>, db_pool: Data<DbPool>, _: UserClaims) -> Result<Json<UserResponse>> {
    let user: UserResponse = users::get_by_id(id.into_inner(), &db_pool).await?.into();

    Ok(Json(user))
}

#[api_v2_operation(tags("User"))]
#[get("/users")]
pub fn get_users(
    db_pool: Data<DbPool>,
    query: Query<UsersQuery>,
    _: UserClaims,
) -> Result<Json<PaginatedUsers>> {
    let total = users::get_count(&db_pool, query.clone().into_inner().into()).await?;
    let members = users::get_all(
        &db_pool,
        query.page.unwrap_or(1),
        query.per_page.unwrap_or(10),
        query.0.into(),
    )
    .await?
    .into_iter()
    .map(|u| u.into())
    .collect();

    Ok(Json(PaginatedUsers { total, members }))
}

async fn send_welcome_email(user_id: i32) {
    actix_web::rt::time::sleep(std::time::Duration::from_secs(3)).await;
    tracing::info!("Sending email to: {user_id}");
}

#[api_v2_operation(tags("User"))]
#[post("/users")]
pub async fn create_user(
    db_pool: Data<DbPool>,
    user_data: Json<NewUserRequest>,
    _: UserClaims,
) -> Result<Json<UserResponse>> {
    let user = users::create(user_data.0.into(), &db_pool).await?;

    // Here is an example of performing a non-blocking background task, such as
    // sending an email, without needing a persistent queue.
    actix_web::rt::spawn(send_welcome_email(user.id));

    Ok(Json(user.into()))
}

#[api_v2_operation(tags("User"))]
#[put("/users/{id}")]
pub async fn update_user(
    db_pool: Data<DbPool>,
    id: Path<i32>,
    user_data: Json<UpdateUserRequest>,
    claims: UserClaims,
) -> Result<Json<UserResponse>> {
    // We can potentially check the user's scopes here to determine if they are
    // permitted to perform this operation.
    tracing::info!("User trying to update: {claims:?}");

    let user = users::update(id.into_inner(), user_data.0.into(), &db_pool).await?;

    Ok(Json(user.into()))
}

#[api_v2_operation(tags("User"))]
#[delete("/users/{id}")]
pub async fn delete_user(db_pool: Data<DbPool>, id: Path<i32>, _: UserClaims) -> Result<NoContent> {
    users::delete(id.into_inner(), &db_pool).await?;

    Ok(NoContent)
}
