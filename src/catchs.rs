use rocket::{
  serde::json::{
      Value,
      json
  },
};

#[catch(401)]
pub fn no_authorized() -> Value {
    json!("Not Aothorized")
}

#[catch(404)]
pub fn no_found() -> Value {
    json!("Not Found")
}