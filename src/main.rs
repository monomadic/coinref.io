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

use iron::prelude::*;

mod models;
mod views;
mod controllers;
mod render_templar;

fn main() {
    let mut router = router::Router::new();
    let mut mount = mount::Mount::new();

    // let coindb = ::models::all();

    // router
    router.get("/", controllers::landing, "index");
    // router.get("/:coin", controllers::coin::MessageHandler{ coindb: coindb }, "coin");
    router.get("/:coin", controllers::coin::show, "coin");
    mount.mount("/", router);

    // static mount
    let static_assets = staticfile::Static::new(::std::path::Path::new("static"));
    mount.mount("/static", static_assets);

    // launch server
    println!("launching server at http://localhost:9000/");
    Iron::new(mount).http("localhost:9000").unwrap();
    println!("done.");

    // fn show(req: &mut Request) -> IronResult<Response> {
    //     use iron::prelude::*;
    //     use iron::status;
    //     use iron::headers::ContentType;

    //     let _coin_name = req.extensions.get::<::router::Router>()
    //         .expect("router to contain extensions")
    //         .find("coin")
    //         .expect(":coin not present in route");

    //     Ok(Response::with((
    //         ContentType::html().0,
    //         status::Ok,
    //         format!("{:?}", req),
    //     )))
    // }
}
