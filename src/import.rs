extern crate coinref;
use coinref::error::*;

extern crate coinmarketcap;

fn main() {
    println!("Importing TOML pages...");

    match read_pages() {
        Ok(_) => println!("done."),
        Err(e) => println!("error: {:?}", e),
    }
}

fn read_pages() -> Result<(), CoinrefError> {
    use std::fs;
    let paths = fs::read_dir("data").unwrap();
    let db = coinref::establish_connection()?;

    println!("requesting coinmarketcap data...");
    let cmc = coinmarketcap::ticker(1000, 0).map_err(|e|
        CoinrefError{ error_type: CoinrefErrorType::ImportError, message: e.message })?;

    // println!("{:?}", cmc);
    let btc = &cmc.iter().find(|c| c.symbol == "BTC").unwrap();
    let btc_mc = btc.market_cap_usd.unwrap();
    // println!("{:?}", neo);

    println!("importing data...");
    for path in paths {
        let filepath = path.unwrap().path();
        let filename = filepath.to_str().unwrap();

        match coinref::template::parse(filename) {
            Ok(mut coin) => {
                if let Some(cmc_coin) = cmc.iter().find(|c| c.symbol == coin.symbol) {
                    println!("applying coinmarketcap data for {}", coin.symbol);

                    if let Some(cap) = cmc_coin.market_cap_usd {
                        coin.market_cap_usd = Some(cap as f32);
                        coin.growth_potential = Some((cap / btc_mc * 100_f64) as f32);
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
