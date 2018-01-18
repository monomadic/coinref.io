extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate templar;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate horrorshow;

// use iron::prelude::*;

pub mod schema;
pub mod models;
mod views;
pub mod controllers;
mod render_templar;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("./database.sql").expect("Error connecting to database.")
}
