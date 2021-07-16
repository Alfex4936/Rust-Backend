#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
// #[macro_use]
// extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::templates::Template;

mod db;
mod routes;
mod utils;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::connection::init_pool())
        .mount(
            "/api",
            routes![
                routes::notice::hello,
                routes::notice::db_test,
                routes::notice::notice_test,
                routes::html::front_test,
            ],
        )
        .attach(Template::fairing())
}

pub fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::Ok, // 챗봇은 무조건 200
        _ => Status::InternalServerError,
    }
}
