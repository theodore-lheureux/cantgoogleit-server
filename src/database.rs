#![allow(dead_code)]

use sqlx::{Pool, Sqlite};

pub mod requests;
pub mod responses;

use responses::ClassroomResponse;

use self::responses::{PositionResponse, UserResponse, ClassroomPositionResponse};

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn create_classroom(
    pool: &Pool<Sqlite>
) -> DBResult<String> {
    // generate random 3 character string
    let id = nanoid::nanoid!(3);

    let res = sqlx::query(
        "DELETE FROM classrooms WHERE created_at < datetime('now', '-5 hour')"
    )
        .execute(pool)
        .await;

    if let Err(e) = res {
        println!("Error: {}", e);
        return Err(e.into());
    }
    

    let classroom = sqlx::query_as::<_, ClassroomResponse>(
        "INSERT INTO classrooms (id) VALUES (?) RETURNING id, created_at"
    )
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(classroom.id)
}

pub async fn create_position(
    pool: &Pool<Sqlite>,
    owner_id: i64,
    classroom_id: String,
) -> DBResult<PositionResponse> {
    let position = sqlx::query_as::<_, responses::PositionResponse>(
        "INSERT INTO positions (owner_id, classroom_id) VALUES (?, ?) RETURNING id, owner_id, created_at"
    )
        .bind(owner_id)
        .bind(classroom_id)
        .fetch_one(pool)
        .await;

    if let Err(e) = position {
        println!("Error: {}", e);
        return Err(e.into());
    }

    let position = position.unwrap();

    Ok(position)
}

pub async fn create_user(
    pool: &Pool<Sqlite>,
    username: String
) -> DBResult<UserResponse> {
    let user = sqlx::query_as::<_, UserResponse>(
        "INSERT INTO users (name) VALUES (?) RETURNING id, name"
    )
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn get_classroom(
    pool: &Pool<Sqlite>,
    classroom_id: String
) -> DBResult<ClassroomResponse> {
    let classroom = sqlx::query_as::<_, ClassroomResponse>(
        "SELECT * FROM classrooms WHERE id = ?"
    )
        .bind(classroom_id)
        .fetch_one(pool)
        .await?;

    Ok(classroom)
}

pub async fn get_position(
    pool: &Pool<Sqlite>,
    user_id: i64,
    classroom_id: String,
) -> DBResult<Option<PositionResponse>> {
    let position = sqlx::query_as::<_, PositionResponse>(
        "SELECT * FROM positions WHERE classroom_id = ? AND owner_id = ?"
    )
        .bind(classroom_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(position)
}

pub async fn get_user(
    pool: &Pool<Sqlite>,
    user_id: i64
) -> DBResult<UserResponse> {
    let user = sqlx::query_as::<_, UserResponse>(
        "SELECT * FROM users WHERE id = ?"
    )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn set_user_name(
    pool: &Pool<Sqlite>,
    user_id: i64,
    name: String
) -> DBResult<UserResponse> {
    let user = sqlx::query_as::<_, UserResponse>(
        "UPDATE users SET name = ? WHERE id = ? RETURNING id, name"
    )
        .bind(name)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn get_classroom_positions(
    pool: &Pool<Sqlite>,
    classroom_id: String
) -> DBResult<Vec<ClassroomPositionResponse>> {
    let positions = sqlx::query_as::<_, ClassroomPositionResponse>(
        "SELECT positions.id, positions.owner_id, positions.created_at, users.name FROM positions INNER JOIN users ON positions.owner_id = users.id WHERE positions.classroom_id = ?"
    )
        .bind(classroom_id)
        .fetch_all(pool)
        .await?;

    Ok(positions)
}