extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

extern crate coinref;

use iron::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    let mut router = router::Router::new();
    let mut mount = mount::Mount::new();
    let db = Arc::new(Mutex::new(coinref::establish_connection().expect("Unable to connect to database")));

    let db_root = db.clone();
    router.get("/", move |r: &mut Request| {
        coinref::controllers::handle_request(coinref::controllers::landing(r, &db_root.lock().unwrap()))
    }, "index");

    let db_coin = db.clone();
    router.get("/:coin", move |r: &mut Request| {
        coinref::controllers::handle_request(
            coinref::controllers::coin::show(r, &db_coin.lock().unwrap()))
    }, "coin");

    mount.mount("/", router);

    // static mount
    let static_assets = staticfile::Static::new(::std::path::Path::new("static"));
    mount.mount("/static", static_assets);

    // launch server
    println!("launching server at http://localhost:9000/");
    iron::Iron::new(mount).http("localhost:9000").unwrap();
    println!("done.");
}