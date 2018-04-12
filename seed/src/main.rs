extern crate coinref;
use coinref::error::*;

extern crate coinmarketcap;
extern crate rusqlite;

pub struct Coin {
    pub id: i32,
    pub name: String,
    pub symbol: String,
}

fn main() {
    use std::path::Path;

    println!("Importing TOML pages...");

    let conn = rusqlite::Connection::open(Path::new("./database.sql"))
        .expect("./database.sql failed to open");

    conn.execute_batch("
         DROP TABLE IF EXISTS coins;
         DROP TABLE IF EXISTS tags;
         DROP TABLE IF EXISTS coin_tags;
    ").expect("error deleting databases");

    conn.execute_batch("

        CREATE TABLE coins (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name VARCHAR NOT NULL,
            symbol VARCHAR NOT NULL,
            website VARCHAR NOT NULL,
            twitter VARCHAR,
            reddit VARCHAR,
            github VARCHAR,
            telegram VARCHAR,
            slack VARCHAR,
            facebook VARCHAR,
            youtube VARCHAR,
            -- instagram VARCHAR,
            -- pinterest VARCHAR,
            -- discord VARCHAR,
            market_cap_usd REAL,
            market_cap_rank INTEGER,
            circulating_supply INTEGER,
            price_in_btc REAL,
            price_in_usd REAL,
            growth_potential REAL,
            page TEXT NOT NULL DEFAULT ''
        );

        CREATE TABLE tags (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name VARCHAR NOT NULL
        );

        CREATE TABLE coin_tags (
          coin_id INTEGER NOT NULL, 
          tag_id INTEGER NOT NULL, 
          PRIMARY KEY (coin_id, tag_id), 
          FOREIGN KEY (coin_id) REFERENCES coins (id) ON DELETE CASCADE, 
          FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
        );

        INSERT INTO tags (name) VALUES ('decentralised');
        INSERT INTO tags (name) VALUES ('shitcoin');
        INSERT INTO coin_tags (coin_id, tag_id) VALUES (1, 1);

    ").expect("error regenerating databases");

    for template in get_coin_templates().expect("coin templates could not be read") {
        println!("{:?}", template);
        conn.execute("INSERT INTO coins (name, symbol, website, twitter)
                      VALUES (?1, ?2, ?3, ?4)",
                      &[
                        &template.name,
                        &template.symbol,
                        &template.website,
                        &template.twitter,
                      ])
        .expect("coin failed to insert");


    }

    let coins = coinref::models::Coin::all(&conn).expect("coins to be selected from the database");

    println!("Successfully imported {} coins.", coins.len());

    for coin in coins {
        println!("{:?}", coin);
    }

    println!("getting BTC: {:?}", coinref::models::Coin::get(&conn, "BTC"));

    // for coin in coins {
    //     print!("{:?}, ", coin);
    // };

    // println!("{:?}", select_coins);

    // let coint_coins = conn.execute("SELECT count(*) FROM coin;", &[]);
    // println!("{:?}", coint_coins);

    // conn.execute("
    //     CREATE TABLE coins (
    //         id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    //         name VARCHAR NOT NULL,
    //         symbol VARCHAR NOT NULL );
    // ", &[]).expect("error thing");

    // match read_pages() {
    //     Ok(_) => println!("done."),
    //     Err(e) => println!("error: {:?}", e),
    // }
}

use coinref::template::CoinTemplate;
fn get_coin_templates() -> Result<Vec<CoinTemplate>, CoinrefError> {
    use std::fs;
    let paths = fs::read_dir("data").unwrap();

    paths.map(|path| {
        let filepath = path.unwrap().path();
        let filename = filepath.to_str().unwrap();
        coinref::template::read(filename)
    }).collect()
}

fn read_pages() -> Result<(), CoinrefError> {
    use std::fs;
    let paths = fs::read_dir("data").unwrap();
    let db = coinref::establish_connection()?;

    println!("requesting coinmarketcap data...");
    let cmc = coinmarketcap::ticker(300, 0).map_err(|e|
        CoinrefError{ error_type: CoinrefErrorType::ImportError, message: e.message })?;

    let btc = &cmc.iter().find(|c| c.symbol == "BTC").unwrap();
    let btc_supply = btc.available_supply.unwrap();
    let btc_price = btc.price_usd.unwrap();

    println!("importing data...");
    for path in paths {
        let filepath = path.unwrap().path();
        let filename = filepath.to_str().unwrap();
        println!("reading {}", filename);

        match coinref::template::parse(filename) {
            Ok(mut coin) => {                
                if let Some(cmc_coin) = cmc.iter().find(|c| c.symbol == coin.symbol) {
                    println!("applying coinmarketcap data for {}", coin.symbol);

                    let alt_price = cmc_coin.price_usd.unwrap();
                    let alt_supply = cmc_coin.available_supply.unwrap();
                    coin.growth_potential = Some(((btc_supply / alt_supply * btc_price) / alt_price) as f32);

                    if let Some(cap) = cmc_coin.market_cap_usd {
                        coin.market_cap_usd = Some(cap as f32);
                    }

                    if let Some(btc) = cmc_coin.price_btc {
                        coin.price_in_btc = Some(btc as f32);
                    }

                    if let Some(usd) = cmc_coin.price_usd {
                        coin.price_in_usd = Some(usd as f32);
                    }

                    if let Some(supply) = cmc_coin.available_supply {
                        coin.circulating_supply = Some(supply as i32);
                    }

                    if let Some(usd) = cmc_coin.price_usd {
                        coin.price_in_usd = Some(usd as f32);
                    }

                    coin.market_cap_rank = Some(cmc_coin.rank as i32);
                }
                coin.insert(&db)?;
                println!("inserted: {}", coin.name);
            },
            Err(error) => println!("error: {}", error.message),
        };
    };

    Ok(())
}
