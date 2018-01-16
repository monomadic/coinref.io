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

mod models;
mod views;
pub mod controllers;
mod render_templar;
