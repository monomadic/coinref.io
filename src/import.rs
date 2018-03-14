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
    let cmc = coinmarketcap::ticker(50, 0).map_err(|e|
        CoinrefError{ error_type: CoinrefErrorType::ImportError, message: e.message })?;

    // println!("{:?}", cmc);
    // println!("{:?}", cmc.into_iter().find(|c| c.symbol == "NEO"));

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
                    }
                }
                coin.insert(&db)?;
                println!("inserted: {}", coin.name);
            },
            Err(error) => println!("error: {}", error.message),
        };
    };

    Ok(())
}
