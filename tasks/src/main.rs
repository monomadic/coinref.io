#[macro_use]
extern crate serde_derive;
extern crate restson;

#[derive(Debug, Serialize, Deserialize)]
struct CoinmarketCapGlobal {
    total_market_cap_usd: f64,
    total_24h_volume_usd: f64,
}

impl restson::RestPath<()> for CoinmarketCapGlobal {
    fn get_path(_: ()) -> Result<String, restson::Error> { Ok("/v1/global/".to_string()) }
}

#[derive(Debug, Serialize, Deserialize)]
struct CoinmarketCap {
    id: String,
    symbol: String,
}

impl restson::RestPath<()> for CoinmarketCap {
    fn get_path(_: ()) -> Result<String, restson::Error> { Ok("/v1/ticker/".to_string()) }
}

fn main() {
    // println!("fetching market cap data...");
    // let mut client = restson::RestClient::new("https://api.coinmarketcap.com").unwrap();
    // let data: CoinmarketCap = client.get(()).unwrap();
    // println!("{:?}", data);
}
