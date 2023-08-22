use database::{requests::UserRequest, responses::{PositionResponse, UserResponse, ClassroomPositionResponse}};
use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    State,
};
use sqlx::{Pool, Sqlite, SqlitePool};

#[macro_use]
extern crate rocket;

mod database;

const COOKIE_NAME: &str = "qid";

#[post("/create_classroom")]
async fn create_classroom(
    _cookies: &CookieJar<'_>,
    pool: &State<Pool<Sqlite>>,
) -> Result<String, Status> {
    if let Ok(id) = database::create_classroom(pool).await {
        Ok(id)
    } else {
        Err(Status::InternalServerError)
    }
}

#[post("/create_position/<classroom_id>")]
async fn create_position(
    cookies: &CookieJar<'_>,
    pool: &State<Pool<Sqlite>>,
    classroom_id: String,
) -> Result<Json<PositionResponse>, Status> {
    let user_id = match cookies.get_private(COOKIE_NAME) {
        Some(id) => id,
        None => { return Err(Status::Unauthorized); },
    };
    let user = database::get_user(pool, user_id.value().parse().unwrap()).await;
    
    if let Err(_) = user {
        return Err(Status::Unauthorized);
    }

    let user = user.unwrap();

    if user.name.is_empty() {
        return Err(Status::BadRequest);
    }
    
    if database::get_position(pool, user.id, classroom_id.clone()).await.unwrap().is_some() {
        return Err(Status::Conflict);
    }   

    if let Ok(res) = database::create_position(pool, user.id, classroom_id).await {
        Ok(Json(res))
    } else {
        Err(Status::InternalServerError)
    }
}

#[get("/get_cookie")]
async fn get_cookie(cookies: &CookieJar<'_>, pool: &State<Pool<Sqlite>>) -> Result<Json<UserResponse>, Status> {
    let user_id = cookies.get_private(COOKIE_NAME);

    match user_id {
        Some(id) => {
            let user = database::get_user(pool, id.value().parse().unwrap()).await;
            match user {
                Ok(user) => Ok(Json(user)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        None => {
            let user = database::create_user(pool, "".to_string()).await;
            match user {
                Ok(user) => {
                    cookies.add_private(Cookie::new(COOKIE_NAME, user.id.to_string()));
                    Ok(Json(user))
                }
                Err(_) => Err(Status::InternalServerError),
            }
        }
    }
}

#[post("/set_username", data = "<username>")]
async fn set_username(
    cookies: &CookieJar<'_>,
    pool: &State<Pool<Sqlite>>,
    username: Form<UserRequest>,
) -> Result<Json<UserResponse>, Status> {
    let username = username.into_inner().name;
    let user_id = cookies.get_private(COOKIE_NAME);

    match user_id {
        Some(id) => {
            let user =
                database::set_user_name(pool, id.value().parse().unwrap(), username.clone()).await;
            match user {
                Ok(user) => Ok(Json(user)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        None => {
            let user = database::create_user(pool, username.clone()).await;
            match user {
                Ok(user) => {
                    cookies.add_private(Cookie::new(COOKIE_NAME, user.id.to_string()));
                    Ok(Json(user))
                }
                Err(_) => Err(Status::InternalServerError),
            }
        }
    }
}

#[get("/get_positions/<classroom_id>")]
async fn get_positions(
    pool: &State<Pool<Sqlite>>,
    classroom_id: String,
) -> Result<Json<Vec<ClassroomPositionResponse>>, Status> {
    let positions = database::get_classroom_positions(pool, classroom_id).await;

    match positions {
        Ok(positions) => Ok(Json(positions)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[launch]
async fn rocket() -> _ {
    let pool = SqlitePool::connect("sqlite://database.db")
        .await
        .expect("Couldn't connect to sqlite database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Couldn't run migrations");

    rocket::build()
        .mount(
            "/",
            routes![create_classroom, create_position, get_cookie, set_username, get_positions],
        )
        .manage(pool)
}
