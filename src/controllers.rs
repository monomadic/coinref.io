use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

use ::error::*;

use diesel::SqliteConnection as DatabaseConnection;

pub fn handle_request(result: Result<String, CoinrefError>) -> IronResult<Response> {
    match result {
        Ok(body) => Ok(Response::with((
            ContentType::html().0,
            status::Ok,
            body,
        ))),
        Err(error) => Err(::iron::error::IronError {
            error: Box::new(error.clone()),
            response: Response::with((
                ContentType::html().0,
                status::Ok,
                error.message,
            )),
        })
    }
}

use iron::Error;
impl From<::diesel::ConnectionError> for CoinrefError {
    fn from(error: ::diesel::ConnectionError) -> Self {
        CoinrefError {
            error_type: CoinrefErrorType::DatabaseConnectionError,
            message: error.description().into(),
        }
    }
}

pub struct Handlers {
    pub db: ::diesel::SqliteConnection,
}

impl Handlers {
    pub fn landing(&self, _req: &mut Request) -> IronResult<Response> {
        // let connection = ::establish_connection();
        let coins = ::models::all_coins(&self.db).unwrap();
        let view = ::views::landing(coins).unwrap();

        Ok(Response::with((
            ContentType::html().0,
            status::Ok,
            view,
        )))
    }
}

pub fn landing(_req: &mut Request, db: &DatabaseConnection) -> Result<String, CoinrefError> {
    let coins = ::models::all_coins(&db)?;
    let view = ::views::landing(coins)?;
    Ok(view)
}

pub fn search(_req: &mut Request) -> IronResult<Response> {
    let connection = ::establish_connection().expect("db connection error");
    let coins = ::models::all_coins(&connection).unwrap();

    Ok(Response::with((
        ContentType::html().0,
        status::Ok,
        ::views::landing(coins).unwrap()
    )))
}

pub mod coin {
    use iron::prelude::*;
    use diesel::SqliteConnection as DatabaseConnection;
    use ::error::*;

    pub fn show(req: &mut Request, db: &DatabaseConnection) -> Result<String, CoinrefError> {
        println!("\nGET {:?}", req);
        let coin_symbol = req.extensions.get::<::router::Router>().unwrap().find("coin").unwrap();
        let coin = ::models::get_coin(&db, coin_symbol)?;
        let body = ::views::coin::show(coin)?;

        Ok(body)
    }
}