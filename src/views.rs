#![allow(unused_variables)]
#![allow(dead_code)]

use horrorshow::prelude::*;

pub fn layout(title: String, content: String) -> String {
    return html! {
        : ::horrorshow::helper::doctype::HTML;
        html {
            head {
                title {: title }
                link(rel="stylesheet", type="text/css", href="/static/style.css", media="all");
                link(rel="stylesheet", href="https://fonts.googleapis.com/css?family=Roboto+Mono");
                link(rel="icon", type="image/png", href="/static/favicon.png");
            }
            body {: Raw(content) }
        }
    }.into_string().unwrap()
}

pub fn header() -> String {
    return html! {
        header {
            a(href="/") { img(src="/static/logo.png", height="31px") }
            : "crypto research network"
        }
    }.into_string().unwrap()
}

pub fn landing(coins: Vec<::models::Coin>) -> String {
    layout(
        format!("coinref.io"),
        html! {
            : Raw(::views::header());
            h2 {: "top coins" }
            table(class="coin-list") {
                @ for coin in coins {
                    tr(onclick=format!("location.href='/{}'", coin.symbol.clone())) {
                        td(class="icon") {
                            img(src=format!("/static/icons/{}.png", coin.symbol));
                        }
                        td(class="name") {
                            a(href=coin.symbol.clone(), class="coin-summary") {: coin.name }
                        }
                    }
                }
            }
        }.into_string().unwrap()
    )
}

pub mod coin {
    use horrorshow::prelude::*;

    pub fn show(coin: ::models::Coin) -> String {
        ::views::layout(
            format!("coinref.io - {}", coin.name),
            html! {
                : Raw(::views::header());
                aside {
                    img(src=format!("/static/icons/{}.png", coin.symbol), class="logo");
                    h1 {: coin.name }
                    div(class="symbol subheading") {: coin.symbol.clone() }
                    div(class="website") {
                        a(href=format!("https://{}", coin.website.clone()), target="_newTab") {: coin.website }
                    }
                    div(class="media_links") {
                        @ if let Some(twitter) = coin.twitter.clone() {
                            a(href=format!("https://twitter.com/{}", twitter), target="_newTab", class="pill-link twitter") {
                                img(src="/static/twitter.png");
                                span {: format!("@{}", twitter) }
                            }
                        }
                    }
                }
                article {
                    : Raw(::render_templar::render_template(&format!("data/{}.templar", coin.symbol)),);
                    h2 {: "Updates" }
                    // @ for news in coin.news {
                    //     li {
                    //         : news.source;
                    //         a(href=news.link, target="_newTab") {: news.link_name }
                    //     }
                    // }
                }
            }.into_string().expect("show() render")
        )
    }
}
