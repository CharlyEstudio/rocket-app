#[macro_use] extern crate rocket;

extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;

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

#[database("mysql")]
pub struct DbConn(diesel::MysqlConnection);

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
        .attach(DbConn::fairing())
        .launch()
        .await;
}
