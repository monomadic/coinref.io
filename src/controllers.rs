use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use rusqlite::Connection as DatabaseConnection;

use ::error::*;

pub fn handle_request(result: Result<String, CoinrefError>) -> IronResult<Response> {
    match result {
        Ok(body) => Ok(Response::with((
            ContentType::html().0,
            status::Ok,
            body,
        ))),
        Err(error) => Err(::iron::error::IronError {
            error: Box::new(error),
            response: Response::with((
                ContentType::html().0,
                status::Ok,
                "iron error".to_string(),
            )),
        })
    }
}

pub fn landing(_req: &mut Request, db: &DatabaseConnection) -> Result<String, CoinrefError> {
    let coins = ::models::Coin::all(&db)?;
    let view = ::views::landing(coins)?;
    Ok(view)
}

// pub fn search(_req: &mut Request) -> IronResult<Response> {
//     let connection = ::establish_connection().expect("db connection error");
//     let coins = ::models::all_coins(&connection).unwrap();

//     Ok(Response::with((
//         ContentType::html().0,
//         status::Ok,
//         ::views::landing(coins).unwrap()
//     )))
// }

pub mod coin {
    use iron::prelude::*;
    use rusqlite::Connection as DatabaseConnection;
    use ::error::*;

    pub fn show(req: &mut Request, db: &DatabaseConnection) -> Result<String, CoinrefError> {
        println!("\nGET {:?}", req);
        let coin_symbol = req.extensions.get::<::router::Router>().unwrap().find("coin").unwrap();
        let coin = ::models::Coin::get(&db, coin_symbol)?;
        let body = ::views::coin::show(coin)?;

        Ok(body)
    }
}