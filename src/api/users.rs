use rocket::get;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String,
    pub age: i32,
}

impl User {
    pub fn new(name: String, age: i32) -> User {
        User { name, age }
    }
}

#[get("/users")]
pub fn index_users() -> Json<Vec<User>> {
    let users = (1..10)
        .map(|i| User::new(format!("user-{}", i), i))
        .collect();
    Json(users)
}

#[get("/users/<id>")]
pub fn get_users(id: usize) -> Json<Option<User>> {
    let users: Vec<_> = (1..10)
        .map(|i| User::new(format!("user-{}", i), i))
        .collect();
    let user = users.into_iter().nth(id);
    Json(user)
}
