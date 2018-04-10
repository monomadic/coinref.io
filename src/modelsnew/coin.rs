pub struct Coin {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub website: String,

    // pub twitter: Option<String>,
    // pub reddit: Option<String>,
    // pub github: Option<String>,
    // pub telegram: Option<String>,
    // pub slack: Option<String>,
    // pub facebook: Option<String>,
    // pub youtube: Option<String>,

    pub market_cap_usd: Option<f32>,
    pub market_cap_rank: Option<i32>,

    pub circulating_supply: Option<i32>,
    pub price_in_btc: Option<f32>,
    pub price_in_usd: Option<f32>,

    pub growth_potential: Option<f32>,

    // pub page: String,

    // pub news: Vec<NewsItem>,
}

