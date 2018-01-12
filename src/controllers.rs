use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

pub fn landing(req: &mut Request) -> IronResult<Response> {
    println!("\nGET {:?}", req);

    Ok(Response::with((
        ContentType::html().0,
        status::Ok,
        ::views::landing(vec!["ETH", "XRB"])
    )))
}

pub mod coin {
    use iron::prelude::*;
    use iron::status;
    use iron::headers::ContentType;

    pub fn show(req: &mut Request) -> IronResult<Response> {
        println!("\nGET {:?}", req);

        let coin_name = req.extensions.get::<::router::Router>().unwrap().find("coin").unwrap();

        let coin = ::models::Coin {
            name:  "Raiblocks".to_string(),
            tag: coin_name.to_string(),
            image: coin_name.to_string(),
            summary: "blah".to_string(),
            website: "https://raiblocks.net".to_string(),
            news: vec![
                ::models::NewsItem {
                    source: "reddit".to_string(),
                    link: "http://reddit.com/".to_string(),
                    link_name: "man does thing".to_string(),
                    time_ago: "2 days ago".to_string(),
                }
            ],
        };

        Ok(Response::with((
            ContentType::html().0,
            status::Ok,
            ::views::coin::show(coin)
        )))
    }
}