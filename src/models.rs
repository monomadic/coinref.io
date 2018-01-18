use diesel::prelude::*;

#[derive(Queryable)]
pub struct Coin {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub website: String,

    pub twitter: Option<String>,
    pub reddit: Option<String>,
    pub github: Option<String>,
    pub telegram: Option<String>,
    pub slack: Option<String>,
    pub facebook: Option<String>,

    pub market_cap: i32,
    pub page: String,

    // pub news: Vec<NewsItem>,
}

pub fn all_coins(connection: SqliteConnection) -> Vec<Coin> {
    use ::schema::coins;
    let c = coins::table.limit(10).load::<::models::Coin>(&connection).expect("Error loading user");

    return c;
}

pub fn get_coin(connection: SqliteConnection, coin_symbol: &str) -> Coin {
    use ::schema::coins;
    let c = coins::table.filter(coins::symbol.eq(coin_symbol)).first::<::models::Coin>(&connection).expect("Error finding coin");

    return c;
}

// #[derive(Debug, Clone, Queryable)]
// pub struct NewsItem {
//     pub source: String,
//     pub link: String,
//     pub link_name: String,
//     pub time_ago: String,
// }

// pub fn top() -> Vec<::models::Coin> {
//     // use diesel::prelude::*;
//     // use schema::coins::dsl::*;
//     // use super::schema::coins;

//     let connection = ::establish_connection();
//     let results = coins.limit(5).load::<Coin>(&connection).expect("Error loading posts");

//     return vec![];
// }
use std::collections::HashMap;
pub fn all() -> HashMap<String, ::models::Coin> {
    return HashMap::new()
//     use toml;
//     use std::collections::HashMap;

//     let coin_db: HashMap<String, ::models::Coin> = toml::from_str(&str_from_file("data/coins.toml")).expect("coins.toml to parse correctly");
//     // println!("coindb initialised from toml. {:?}", coin_db);
//     println!("read coindb.toml");
//     return coin_db;

//     fn str_from_file(file: &str) -> String {
//         use std::io::prelude::*;
//         let mut handle = ::std::fs::File::open(file).expect("file to open");
//         let mut bytebuffer = Vec::new();
//         handle.read_to_end(&mut bytebuffer).expect("text file to read");
//         return String::from_utf8(bytebuffer).expect("file to convert from utf8")
//     }
}
