mod api;
use crate::api::users;
use rocket::{launch, routes};

//#[macro_use]
//extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![users::index_users, users::get_users])
}
