use ::error::*;

#[derive(Debug)]
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
    pub youtube: Option<String>,

    pub market_cap_usd: Option<f32>,
    pub market_cap_rank: Option<i32>,

    pub circulating_supply: Option<i32>,
    pub price_in_btc: Option<f32>,
    pub price_in_usd: Option<f32>,

    pub growth_potential: Option<f32>,

    pub page: String,

    // pub news: Vec<NewsItem>,
}

#[derive(Debug)]
pub struct NewCoin {
    pub name: String,
    pub symbol: String,
    pub website: String,

    pub twitter: Option<String>,
    pub reddit: Option<String>,
    pub github: Option<String>,
    pub telegram: Option<String>,
    pub slack: Option<String>,
    pub facebook: Option<String>,
    pub youtube: Option<String>,

    pub market_cap_usd: Option<f32>,
    pub market_cap_rank: Option<i32>,

    pub circulating_supply: Option<i32>,
    pub price_in_btc: Option<f32>,
    pub price_in_usd: Option<f32>,

    pub growth_potential: Option<f32>,
    pub page: String,
}

impl NewCoin {
    pub fn insert(&self, db: &rusqlite::Connection) -> Result<i32, CoinrefError> {
        // Ok(::diesel::insert_into(coins::table)
        //     .values(self)
        //     .execute(db)?)
        Ok(db.execute("INSERT INTO coins (name, symbol, website, twitter)
          VALUES (?1, ?2, ?3, ?4)",
          &[
            &self.name,
            &self.symbol,
            &self.website,
            &self.twitter,
          ])?
        )
    }
}

use rusqlite;
impl Coin {
    pub fn all(db: &rusqlite::Connection) -> Result<Vec<Coin>, CoinrefError> {
        let mut select_coins = db.prepare("SELECT * FROM coins").expect("select from coin");
        let results: Result<Vec<Coin>, rusqlite::Error> = select_coins.query_map(&[], |row|
            Coin {
                id: row.get(0),
                name: row.get(1),
                symbol: row.get(2),
                website: row.get(3),
                twitter: None,
                reddit: None,
                github: None,
                telegram: None,
                slack: None,
                facebook: None,
                youtube: None,
                market_cap_usd: None,
                market_cap_rank: None,
                circulating_supply: None,
                price_in_btc: None,
                price_in_usd: None,
                growth_potential: None,
                page: "".to_string(),
            }
        ).expect("select coins to return valid entries").collect();
        Ok(results.unwrap())
    }

    pub fn get(db: &rusqlite::Connection, coin_symbol: &str) -> Result<Coin, CoinrefError> {
        let mut select_coin = db.prepare("SELECT * FROM coins WHERE symbol=?1 LIMIT 1").expect("select from coin");
        let coin:Coin = select_coin.query_row(&[&coin_symbol], |row| {
            Coin {
                id: row.get(0),
                name: row.get(1),
                symbol: row.get(2),
                website: row.get(3),
                twitter: None,
                reddit: None,
                github: None,
                telegram: None,
                slack: None,
                facebook: None,
                youtube: None,
                market_cap_usd: None,
                market_cap_rank: None,
                circulating_supply: None,
                price_in_btc: None,
                price_in_usd: None,
                growth_potential: None,
                page: "".to_string(),
            }
        }).expect("symbol not found");

        Ok(coin)
    }

    // pub fn tags(&self, conn: &SqliteConnection) -> Vec<Tag> {
    //     let coin_tag_ids = CoinTag::belonging_to(self).select(coin_tags::tag_id);

    //     println!("{:?}", tags::id.eq(coin_tag_ids));

    //     tags::table
    //         .filter(tags::id.eq(coin_tag_ids))
    //         .load::<Tag>(conn)
    //         .expect("could not load tags")
    // }

    // pub fn find_by_symbol(connection: &SqliteConnection, symbol: &str) -> Self {
    //     use ::schema::coins;
    //     coins::table
    //         .filter(coins::symbol.eq(symbol.to_string()))
    //         .first::<::models::Coin>(connection)
    //         .expect(&format!("Error finding coin: {}", symbol))
    // }

    // pub fn count(connection: )
}

#[derive(Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

pub struct CoinTag {
    pub id: i32,
    pub coin_id: i32,
    pub tag_id: i32,
}

// pub fn all_coins(connection: &SqliteConnection) -> Result<Vec<Coin>, CoinrefError> {
//     Ok(coins::table
//         .limit(100)
//         .order(coins::market_cap_rank)
//         .load::<::models::Coin>(connection)?
//     )
// }

// pub fn get_coin(connection: &SqliteConnection, coin_symbol: &str) -> Result<Coin, CoinrefError> {
//     Ok(coins::table.filter(coins::symbol.eq(coin_symbol)).first::<::models::Coin>(connection)?)
// }

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
// use std::collections::HashMap;
// pub fn all() -> HashMap<String, ::models::Coin> {

//     let mut select_coins = conn.prepare("SELECT name FROM coins;").expect("select from coin");

//     let coins = select_coins.query_map(&[], |row| {
//         Coin {
//             id: row.get(0),
//             name: row.get(1),
//             time_created: row.get(2),
//             data: row.get(3)
//         }
//     }).expect("select coins to return valid entries");

//     return HashMap::new()
// //     use toml;
// //     use std::collections::HashMap;

// //     let coin_db: HashMap<String, ::models::Coin> = toml::from_str(&str_from_file("data/coins.toml")).expect("coins.toml to parse correctly");
// //     // println!("coindb initialised from toml. {:?}", coin_db);
// //     println!("read coindb.toml");
// //     return coin_db;

// //     fn str_from_file(file: &str) -> String {
// //         use std::io::prelude::*;
// //         let mut handle = ::std::fs::File::open(file).expect("file to open");
// //         let mut bytebuffer = Vec::new();
// //         handle.read_to_end(&mut bytebuffer).expect("text file to read");
// //         return String::from_utf8(bytebuffer).expect("file to convert from utf8")
// //     }
// }
