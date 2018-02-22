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
    let db = coinref::establish_connection();

    // let coins = paths.into_iter().fold_results(|path| {
    //     let filename = path.unwrap().path();
    //     let coin = coinref::template::parse(filename.to_str().unwrap())

    // }).collect();

    // let inserted_coins = insert_into(coins)
    //     .values(coins)
    //     .get_results(&conn)?;

    // Ok(())

    for path in paths {
        let filename = path.unwrap().path();
        // println!("Name: {}", path.unwrap().path().display());

        let coin = coinref::template::parse(filename.to_str().unwrap())?;
        coin.insert(&db)?;



        // println!("{}", template);


    };

    Ok(())
}
