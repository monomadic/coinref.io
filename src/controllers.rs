use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

pub fn landing(req: &mut Request) -> IronResult<Response> {
    println!("\nGET {:?}", req);

    let coindb = ::models::all();

    Ok(Response::with((
        ContentType::html().0,
        status::Ok,
        ::views::landing(coindb)
    )))
}

pub mod coin {
    use std::collections::HashMap;
    use iron::prelude::*;
    use iron::headers::ContentType;
    use iron::{ status, Handler };

    // pub struct MessageHandler {
    //     pub coindb: HashMap<String, ::models::Coin>,
    // }

    // impl Handler for MessageHandler {
    //     fn handle(&self, req: &mut Request) -> IronResult<Response> {
    //         println!("\nGET {:?}", req);
    //         let coin_name = req.extensions.get::<::router::Router>().unwrap().find("coin").unwrap();
    //         let coin = self.coindb.get(coin_name).unwrap();

    //         Ok(Response::with((
    //             ContentType::html().0,
    //             status::Ok,
    //             ::views::coin::show(coin.clone())
    //         )))
    //     }
    // }

    pub fn show(req: &mut Request) -> IronResult<Response> {
        println!("\nGET {:?}", req);

        let coin_name = req.extensions.get::<::router::Router>().unwrap().find("coin").unwrap();

        let coindb = ::models::all();
        let coin = coindb.get(coin_name).unwrap();

        // let coin = ::models::Coin {
        //     name:  "Raiblocks".to_string(),
        //     tag: coin_name.to_string(),
        //     image: coin_name.to_string(),
        //     summary: ::render_templar::render_template(&format!("data/{}.templar", coin_name)),
        //     website: "https://raiblocks.net".to_string(),
        //     news: vec![
        //         ::models::NewsItem {
        //             source: "reddit".to_string(),
        //             link: "http://reddit.com/".to_string(),
        //             link_name: "man does thing".to_string(),
        //             time_ago: "2 days ago".to_string(),
        //         }
        //     ],
        // };

        Ok(Response::with((
            ContentType::html().0,
            status::Ok,
            ::views::coin::show(coin.clone())
        )))
    }
}