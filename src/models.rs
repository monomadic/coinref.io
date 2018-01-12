
pub struct Coin {
    pub name: String,
    pub tag: String,
    pub image: String,
    pub summary: String,
    pub website: String,
    pub news: Vec<NewsItem>,
}

pub struct NewsItem {
    pub source: String,
    pub link: String,
    pub link_name: String,
    pub time_ago: String,
}
