extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

extern crate coinref;

fn main() {
    let mut router = router::Router::new();
    let mut mount = mount::Mount::new();

    // let coindb = ::models::all();

    // use coinref::models::Coin;

    // use coinref::schema::coins;
    // use diesel::prelude::*;

    let connection = coinref::establish_connection();
    // let results = coins::table.load::<coinref::models::Coin>(&connection).expect("Error loading posts");
    // coins::table.find(1).first::<coinref::models::Coin>(&connection).expect("Error loading user");

    // println!("{:?}", results);

    // router
    router.get("/", coinref::controllers::landing, "index");
    // router.get("/:coin", controllers::coin::MessageHandler{ coindb: coindb }, "coin");
    router.get("/:coin", coinref::controllers::coin::show, "coin");
    mount.mount("/", router);

    // static mount
    let static_assets = staticfile::Static::new(::std::path::Path::new("static"));
    mount.mount("/static", static_assets);

    // launch server
    println!("launching server at http://localhost:9000/");
    iron::Iron::new(mount).http("localhost:9000").unwrap();
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