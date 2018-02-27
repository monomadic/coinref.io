extern crate coinref;
use coinref::error::*;

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

    for path in paths {
        let filepath = path.unwrap().path();
        let filename = filepath.to_str().unwrap();

        match coinref::template::parse(filename) {
            Ok(coin) => { coin.insert(&db)?; println!("inserted: {}", coin.name); },
            Err(error) => println!("error: {}", error.message),
        };
    };

    Ok(())
}
