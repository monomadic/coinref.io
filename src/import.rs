extern crate coinref;

use coinref::error::*;
// use coinref::models::*;

fn main() {
    println!("Importing TOML pages...");

    match read_pages() {
        Ok(_) => println!("done."),
        Err(e) => println!("error: {:?}", e),
    }
}

fn read_pages() -> Result<(), CoinrefError> {
    use std::fs;
    // use diesel::select;
    // use ::coinref::schema::coins::dsl::*;

    let paths = fs::read_dir("data").unwrap();
    let db = coinref::establish_connection()?;

    // let coins = paths.into_iter().fold_results(|path| {
    //     let filename = path.unwrap().path();
    //     let coin = coinref::template::parse(filename.to_str().unwrap())

    // }).collect();

    // let inserted_coins = insert_into(coins)
    //     .values(coins)
    //     .get_results(&conn)?;

    // Ok(())

    for path in paths {
        let filepath = path.unwrap().path();
        let filename = filepath.to_str().unwrap();
        // println!("Name: {}", filename.display());

        // let coin = coinref::template::parse(filename.to_str().unwrap())?;

        match coinref::template::parse(filename) {
            Ok(coin) => { coin.insert(&db)?; println!("inserted: {}", coin.name); },
            Err(error) => println!("error: {}", error.message),
        };

        // coin.insert(&db)?;

        // println!("{}", template);


    };

    Ok(())
}
