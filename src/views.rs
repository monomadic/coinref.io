#![allow(unused_variables)]
#![allow(dead_code)]

use horrorshow::prelude::*;
use error::*;

impl From<::horrorshow::Error> for CoinrefError {
    fn from(error: ::horrorshow::Error) -> Self {
        CoinrefError {
            error_type: CoinrefErrorType::ViewError,
            message: format!("horrowshow error: {:?}", error),
        }
    }
}

pub fn layout(title: String, content: String) -> String {
    return html! {
        : ::horrorshow::helper::doctype::HTML;
        html {
            head {
                title {: title }
                link(rel="stylesheet", type="text/css", href="/static/style.css?v=1", media="all");
                link(rel="stylesheet", href="https://fonts.googleapis.com/css?family=Roboto+Mono");
                link(rel="icon", type="image/png", href="/static/favicon.png");
            }
            body {: Raw(content) }
        }
    }.into_string().expect("view compile: layout()")
}

pub fn header() -> Result<String, CoinrefError> {
    return Ok(html! {
        header {
            a(href="/") { img(src="/static/logo.png", height="31px") }
            : "crypto research database"
        }
    }.into_string()?)
}

pub fn landing(coins: Vec<::models::Coin>) -> Result<String, CoinrefError> {
    let header = ::views::header()?;
    // let market_cap_usd = format!("{}", coin.market_cap_usd);

    Ok(layout(
        format!("coinref.io"),
        html! {
            : Raw(header);
            h2 {: "top coins" }
            table(class="coin-list") {
                @ for coin in coins {
                    tr(onclick=format!("location.href='/{}'", coin.symbol.clone())) {
                        td(class="icon") {
                            div(class="coin") {
                                img(src=format!("/static/icons/{}.png", coin.symbol));
                            }
                        }
                        td(class="name") {
                            a(href=coin.symbol.clone(), class="coin-summary") {: coin.name }
                        }
                        td(class="cap") {
                            span {: "$" }
                            span {: ::separator::separate(&coin.market_cap_usd.unwrap_or(0.0).to_string(), ',') }
                        }
                    }
                }
            }
        }.into_string()?
    ))
}

pub mod coin {
    use horrorshow::prelude::*;
    use error::*;

    pub fn show(coin: ::models::Coin) -> Result<String, CoinrefError> {
        let header = ::views::header()?;

        let page_html = ::template::parse(&format!("data/{}.templar", coin.symbol))?.page;
        // let page_html = ::template::render(&format!("data/{}.templar", coin.symbol))?;

        Ok(::views::layout(
            format!("coinref.io - {}", coin.name),
            html! {
                : Raw(header);
                aside {
                    div(class="coin big-coin") {    
                        img(src=format!("/static/icons/{}.png", coin.symbol), class="logo");
                    }
                    h1 {: coin.name }
                    div(class="symbol subheading") {: coin.symbol.clone() }
                    div(class="website") {
                        a(href=format!("https://{}", coin.website.clone()), target="_newTab") {: coin.website }
                    }
                    div(class="media_links") {
                        @ if let Some(twitter) = coin.twitter.clone() {
                            a(href=format!("https://twitter.com/{}", twitter), target="_newTab", class="pill-link icon-twitter") {
                                span {: format!("@{}", twitter) }
                            }
                        }
                        @ if let Some(reddit) = coin.reddit.clone() {
                            a(href=format!("https://reddit.com/{}", reddit), target="_newTab", class="pill-link icon-reddit") {
                                span {: format!("@{}", reddit) }
                            }
                        }
                        @ if let Some(github) = coin.github.clone() {
                            a(href=format!("https://github.com/{}", github), target="_newTab", class="pill-link icon-github") {
                                span {: format!("@{}", github) }
                            }
                        }
                    }
                }
                article {
                    div(class="page") { : Raw(page_html) }
                    // div(class="page") { : Raw(coin.page) }
                    h2 {: "Updates" }
                    // @ for news in coin.news {
                    //     li {
                    //         : news.source;
                    //         a(href=news.link, target="_newTab") {: news.link_name }
                    //     }
                    // }
                }
            }.into_string()?
        ))
    }
}
