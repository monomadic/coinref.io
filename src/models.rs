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
