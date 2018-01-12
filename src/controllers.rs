use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

pub fn landing(req: &mut Request) -> IronResult<Response> {
    println!("\nGET {:?}", req);

    Ok(Response::with((
        ContentType::html().0,
        status::Ok,
        ::views::landing(vec!["eth"])
    )))
}

pub mod coin {
    use iron::prelude::*;
    use iron::status;
    use iron::headers::ContentType;

    pub fn show(req: &mut Request) -> IronResult<Response> {
        println!("\nGET {:?}", req);

        let coin_name = req.extensions.get::<::router::Router>().unwrap().find("coin").unwrap();
        // println!("{:?}", req.extensions.get::<router::Router>());

        let coin = ::models::Coin {
            name:  coin_name.to_string(),
            tag: coin_name.to_string(),
            image: coin_name.to_string(),
            summary: "blah".to_string(),
        };

        Ok(Response::with((
            ContentType::html().0,
            status::Ok,
            ::views::coin::show(coin)
        )))
    }
}