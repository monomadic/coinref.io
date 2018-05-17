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

pub fn youtube_video(id: &str) -> String {
    let thumbnail_url = format!("https://img.youtube.com/vi/{}/0.jpg", id);

    return html! {
        img(src=thumbnail_url)
    }.into_string().expect("view compile: layout()")
}

pub fn layout(title: String, content: String) -> Result<String, CoinrefError> {
    return Ok(html! {
        : ::horrorshow::helper::doctype::HTML;
        html {
            head {
                title {: title }

                meta(http-equiv="X-UA-Compatible", content="chrome=1");
                meta(name="viewport", content="initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=0");
                meta(name="apple-touch-fullscreen", content="yes");
                meta(name="apple-mobile-web-app-capable", content="yes");

                link(rel="stylesheet", type="text/css", href="/static/style.css?v=3", media="all");
                link(rel="stylesheet", href="https://fonts.googleapis.com/css?family=Roboto+Mono");
                link(rel="icon", type="image/png", href="/static/favicon.png");
            }
            body(class="desktop-constrain padding-horizontal-m") {: Raw(content) }
        }
    }.into_string()?)
}

pub fn header() -> Result<String, CoinrefError> {
    return Ok(html! {
        header {
            // : Raw(youtube_video("5PsQPpFgvu4"));
            a(href="/") { img(src="/static/logo.png", height="31px") }
            span(class="no-wrap") {: "crypto research reports" }
        }
    }.into_string()?)
}

pub fn landing(coins: Vec<::models::Coin>) -> Result<String, CoinrefError> {
    let header = ::views::header()?;

    Ok(layout(
        format!("coinref.io"),
        html! {
            : Raw(header);
            // @ for category in ["social", "wallet"].iter() {
            //     div(class="coin-grid grid clear") {
            //         div(class="grid-heading") {: category }
            //         @ for coin in coins.iter().filter(|c| c.tags.contains(&category.to_string())) {
            //             div(class="grid-item padding-s") {
            //                 div(class="icon") {
            //                     div(class="coin") {
            //                         a(href=format!("/{}", coin.symbol)) {
            //                             img(src=format!("/static/icons/{}.png", coin.symbol));
            //                         }
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }
            div(class="coin-grid grid clear") {
                @ for coin in coins {
                    div(class="coin grid-item padding-s", data-symbol=coin.symbol.clone()) {
                        div(class="icon") {
                            a(href=format!("/{}", coin.symbol.clone())) {
                                img(src=format!("/static/icons/{}.png", coin.symbol.clone()));
                            }
                        }
                    }
                }
            }
            // table(class="coin-list") {
            //     tr(class="heading") {
            //         th {: "Rank" }
            //         th(colspan="2") { }
            //         th {: "Growth Potential" }
            //         th {: "Supply" }
            //         th {: "Price (BTC)" }
            //         th {: "Price (USD)" }
            //         th {: "Cap" }
            //     }
            //     @ for coin in coins {
            //         tr(class="data", onclick=format!("location.href='/{}'", coin.symbol.clone())) {
            //             td(class="rank") {: coin.market_cap_rank }
            //             td(class="icon") {
            //                 div(class="coin") {
            //                     img(src=format!("/static/icons/{}.png", coin.symbol));
            //                 }
            //             }
            //             td(class="name") {
            //                 a(href=coin.symbol.clone(), class="coin-summary") {: coin.name }
            //             }
            //             td(class="growth_potential") {: format!("{:.0}x", coin.growth_potential.unwrap_or(0.0)) }
            //             td(class="circulating_supply") {: ::separator::number(coin.circulating_supply) }
            //             td(class="price_in_btc") {: ::separator::price_btc(coin.price_in_btc) }
            //             td(class="price_in_usd") {: ::separator::price_dollar(coin.price_in_usd) }
            //             td(class="cap") {: ::separator::number(coin.market_cap_usd) }
            //         }
            //     }
            // }
        }.into_string()?
    )?)
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
                aside(class="desktop-only") {
                    div(class="coin big-coin") {    
                        img(src=format!("/static/icons/{}.png", coin.symbol), class="logo");
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
                        @ if let Some(facebook) = coin.facebook.clone() {
                            a(href=format!("https://facebook.com/groups/{}", facebook), target="_newTab", class="pill-link icon-facebook") {
                                span {: format!("@{}", facebook) }
                            }
                        }
                        @ if let Some(youtube) = coin.youtube.clone() {
                            a(href=format!("https://www.youtube.com/channel/{}", youtube), target="_newTab", class="pill-link icon-youtube") {
                                span {: format!("@{}", youtube) }
                            }
                        }
                    }
                    br {}
                    div(class="stats") {
                        div(class="cmc") {
                            strong {: "Market Cap: " }
                            span {: ::separator::number(coin.market_cap_usd) }
                        }
                        br {}
                        div(class="cmc") {
                            strong {: "Circulating Supply: " }
                            span {: ::separator::number(coin.circulating_supply) }
                        }
                        br {}
                        div(class="cmc") {
                            strong {: "Price USD: " }
                            span {: ::separator::number(coin.price_in_usd) }
                        }
                        br {}
                        div(class="cmc") {
                            strong {: "Price BTC: " }
                            span {: ::separator::number(coin.price_in_btc) }
                        }
                    }
                }
                article {
                    div(class="coin big-coin mobile-only") {    
                        img(src=format!("/static/icons/{}.png", coin.symbol.clone()), class="logo center")
                    }
                    h1 {
                        : coin.name
                    }
                    div(class="symbol subheading") {: coin.symbol.clone() }
                    div(class="website") {
                        a(href=format!("https://{}", coin.website.clone()), target="_newTab") {: coin.website }
                    }
                    div(class="tag-list") {
                        @ for tag in coin.tags {
                            a(href=format!("/tag/{}", tag.clone()), target="_newTab", class="tag margin-side-xs margin-vertical-s") {: tag }
                        }
                    }
                    div(class="page") {: Raw(page_html) }
                    // div(class="page") { : Raw(coin.page) }
                    // h2 {: "Updates" }
                    // @ for news in coin.news {
                    //     li {
                    //         : news.source;
                    //         a(href=news.link, target="_newTab") {: news.link_name }
                    //     }
                    // }

                }
            }.into_string()?
        )?)
    }
}
