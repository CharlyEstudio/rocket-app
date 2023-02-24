use crate::auth::BasicAuth;
use rocket::{
    serde::json::{
        Value,
        json
    },
    response::status,
};

#[get("/rustaceans")]
pub fn get_rustanceans(_auth: BasicAuth) -> Value {
    json!([{"id": 1, "name": "Charly"}, {"id": 1, "name": "Bere"}])
}

#[get("/rustaceans/<id>")]
pub fn view_rustancean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Charly", "email": "hola@hola.com"})
}

#[post("/rustaceans", format = "json")]
pub fn create_rustancean(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "Yasmin", "email": "hola@hola.com"})
}

#[put("/rustaceans/<id>", format = "json")]
pub fn update_rustancean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Charly", "email": "hola@hola.com"})
}

#[delete("/rustaceans/<_id>")]
pub fn delete_rustancean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}