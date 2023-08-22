use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ClassroomRequest {
    pub id: String
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PositionRequest {
    pub name: String
}

#[derive(Deserialize, Debug, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct UserRequest {
    pub name: String
}