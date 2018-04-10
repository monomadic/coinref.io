extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate templar;

#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate horrorshow;

// use iron::prelude::*;

extern crate rusqlite;

pub mod models;
pub mod views;
pub mod controllers;
pub mod template;
pub mod error;

mod separator;

// pub fn establish_connection() -> Result<SqliteConnection, diesel::ConnectionError> {
//     Ok(SqliteConnection::establish("./database.sql")?)
// }
