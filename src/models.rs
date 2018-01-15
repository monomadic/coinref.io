use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Coin {
    pub name: String,
    pub tag: String,
    pub website: String,
    pub news: Vec<NewsItem>,
    pub twitter: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewsItem {
    pub source: String,
    pub link: String,
    pub link_name: String,
    pub time_ago: String,
}

pub fn all() -> HashMap<String, ::models::Coin> {
    use toml;
    use std::collections::HashMap;

    let coin_db: HashMap<String, ::models::Coin> = toml::from_str(&str_from_file("data/coins.toml")).expect("coins.toml to parse correctly");
    // println!("coindb initialised from toml. {:?}", coin_db);
    println!("read coindb.toml");
    return coin_db;

    fn str_from_file(file: &str) -> String {
        use std::io::prelude::*;
        let mut handle = ::std::fs::File::open(file).expect("file to open");
        let mut bytebuffer = Vec::new();
        handle.read_to_end(&mut bytebuffer).expect("text file to read");
        return String::from_utf8(bytebuffer).expect("file to convert from utf8")
    }
}
