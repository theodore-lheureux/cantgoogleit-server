use rocket::serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ClassroomResponse {
    pub id: String,
    pub created_at: String,
}

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PositionResponse {
    pub id: i64,
    pub owner_id: i64,
    pub created_at: String,
}

#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
}


#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ClassroomPositionResponse {
    pub id: i64,
    pub owner_id: i64,
    pub created_at: String,
    pub name: String,
}
