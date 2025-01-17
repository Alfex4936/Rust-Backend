#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate chrono;

use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod db;
mod kakao_json;
mod routes;
mod utils;

pub const MY_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36";

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        // .manage(db::connection::init_pool())
        .mount(
            "/v1",
            routes![
                routes::notice::hello,
                routes::notice::db_test,
                routes::notice::get_notices,
                routes::notice::notice_test,
                routes::notice::last_notice_test,
                routes::notice::json_test,
                routes::html::front_test,
                routes::html::just_test,
                routes::html::kakao_test,
            ],
        )
        .mount("/", StaticFiles::from("src/templates"))
        .attach(Template::fairing())
}

pub fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::Ok, // 챗봇은 무조건 200
        _ => Status::InternalServerError,
    }
}
