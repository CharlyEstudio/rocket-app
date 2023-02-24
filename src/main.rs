#[macro_use] extern crate rocket;

mod catchs;
mod auth;
mod rustaceans;

use rustaceans::{
    get_rustanceans,
    view_rustancean,
    create_rustancean,
    update_rustancean,
    delete_rustancean,
};
use catchs::{
    no_found,
    no_authorized,
};

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustanceans,
            view_rustancean,
            create_rustancean,
            update_rustancean,
            delete_rustancean,
        ])
        .register("/", catchers![
            no_found,
            no_authorized,
        ])
        .launch()
        .await;
}
