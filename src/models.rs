use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// mod schema {
//     infer_schema!("database.sql");
// }
use ::schema::*;
use ::schema::coins;
use ::error::*;

#[derive(Queryable, Associations, Identifiable)]
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

    pub market_cap_usd: Option<f32>,
    pub market_cap_rank: Option<i32>,

    pub circulating_supply: Option<i32>,
    pub price_in_btc: Option<f32>,
    pub price_in_usd: Option<f32>,

    pub growth_potential: Option<f32>,

    pub page: String,

    // pub news: Vec<NewsItem>,
}

#[derive(Debug, Insertable)]
#[table_name = "coins"]
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

    pub market_cap_usd: Option<f32>,
    pub market_cap_rank: Option<i32>,

    pub circulating_supply: Option<i32>,
    pub price_in_btc: Option<f32>,
    pub price_in_usd: Option<f32>,

    pub growth_potential: Option<f32>,
    pub page: String,
}

impl NewCoin {
    pub fn insert(&self, db: &SqliteConnection) -> Result<usize, ::diesel::result::Error> {
        Ok(::diesel::insert_into(coins::table)
            .values(self)
            .execute(db)?)
    }
}

impl Coin {
    pub fn tags(&self, conn: &SqliteConnection) -> Vec<Tag> {
        // use diesel::sqlite::expression::dsl::any;

        let coin_tag_ids = CoinTag::belonging_to(self).select(coin_tags::tag_id);

        println!("{:?}", tags::id.eq(coin_tag_ids));

        tags::table
            .filter(tags::id.eq(coin_tag_ids))
            .load::<Tag>(conn)
            .expect("could not load tags")
    }

    pub fn find_by_symbol(connection: &SqliteConnection, symbol: &str) -> Self {
        use ::schema::coins;
        coins::table
            .filter(coins::symbol.eq(symbol.to_string()))
            .first::<::models::Coin>(connection)
            .expect(&format!("Error finding coin: {}", symbol))
    }
}

#[derive(Queryable, Associations, Identifiable, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Coin)]
#[belongs_to(Tag)]
pub struct CoinTag {
    pub id: i32,
    pub coin_id: i32,
    pub tag_id: i32,
}

pub fn all_coins(connection: &SqliteConnection) -> Result<Vec<Coin>, CoinrefError> {
    Ok(coins::table
        .limit(100)
        .order(coins::market_cap_rank)
        .load::<::models::Coin>(connection)?
    )
}

pub fn get_coin(connection: &SqliteConnection, coin_symbol: &str) -> Result<Coin, CoinrefError> {
    Ok(coins::table.filter(coins::symbol.eq(coin_symbol)).first::<::models::Coin>(connection)?)
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
