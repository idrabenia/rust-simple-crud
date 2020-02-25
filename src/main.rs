#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;

use self::rocket_hello::*;
use self::models::{User, NewUser, UpdateUser};
use self::diesel::prelude::*;
use std::result::Result;
use rocket::http::Status;
use rocket_hello::schema::users::dsl::*;
use rocket_hello::schema::users::dsl;
use rocket_hello::connection;
use connection::DbConn;
use log::{warn, error};
use diesel::result::Error;

#[macro_use] extern crate rocket;
extern crate rocket_hello;
extern crate diesel;

#[get("/<id_val>")]
fn index(id_val: i32, connection: DbConn) -> res!(User) {
    let results = users.find(id_val)
        .get_result::<User>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[get("/")]
fn list(connection: DbConn) -> res_vec!(User) {
    let results = users.load::<User>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[post("/<id_val>", format = "application/json", data = "<user>")]
fn update(id_val: i32, user: Json<UpdateUser>, connection: DbConn) -> res!(User) {
    let results = diesel::update(users)
        .filter(dsl::id.eq(id_val))
        .set(&user.0)
        .get_result::<User>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[post("/", format = "application/json", data = "<user>")]
fn create(user: Json<NewUser>, connection: DbConn) -> res!(User) {
    let results = diesel::insert_into(users)
        .values(&user.0)
        .get_result::<User>(&*connection);

    match results {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err(on_error(&e))
    }
}

#[delete("/<id_val>")]
fn delete(id_val: i32, conn: DbConn) -> Status {
    let result = diesel::delete(users)
        .filter(dsl::id.eq(id_val))
        .execute(&*conn);

    match result {
        Ok(val) => if val > 0 { Status::Ok } else { Status::NotFound },
        Err(e) => on_error(&e)
    }
}

fn on_error(e: &Error) -> Status {
    match e {
        Error::NotFound => {
            warn!("Entity not found {}", e);
            Status::NotFound
        },

        _ => {
            error!("Error during request processing {}", e);
            Status::InternalServerError
        }
    }
}

fn main() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![index, create, list, update, delete])
        .launch();
}
