extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

#[macro_use]
extern crate horrorshow;

use iron::prelude::*;

mod models;
mod views;
mod controllers;

fn main() {
    let mut router = router::Router::new();
    let mut mount = mount::Mount::new();

    // router
    router.get("/", controllers::landing, "index");
    router.get("/:coin", controllers::coin::show, "coin");
    mount.mount("/", router);

    // static mount
    let static_assets = staticfile::Static::new(::std::path::Path::new("static"));
    mount.mount("/static", static_assets);

    // launch server
    println!("launching server at http://localhost:9000/");
    Iron::new(mount).http("localhost:9000").unwrap();
    println!("done.");
}
