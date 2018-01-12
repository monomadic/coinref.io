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
            }
            body {: Raw(content) }
        }
    }.into_string().unwrap()
}

pub fn header() -> String {
    return html! {
        header {
            a(href="/") { img(src="/static/logo.png", height="31px") }
            : "crypto research made simple"
        }
    }.into_string().unwrap()
}

pub fn landing(coins: Vec<&str>) -> String {
    layout(
        format!("conref.io"),
        html! {
            : Raw(::views::header());
            @ for coin in coins {
                li {
                    a(href=coin) {: coin}
                }
            }
        }.into_string().unwrap()
    )
}

pub mod coin {
    use horrorshow::prelude::*;

    pub fn show(coin: ::models::Coin) -> String {
        ::views::layout(
            format!("conref.io - {}", coin.name),
            html! {
                : Raw(::views::header());
                aside {
                    img(src=format!("/static/icons/{}.png", coin.image));
                    h1 {: coin.name }
                    div(class="tag") {: coin.tag }
                    div(class="website") {
                        a(href=coin.website.clone()) {: coin.website }
                    }
                }
                article {
                    h2 {: "Summary" }
                    p {: coin.summary }
                    h2 {: "News" }
                    @ for news in coin.news {
                        li {
                            a(href=news.link) {: news.link_name }
                        }
                    }
                }
            }.into_string().unwrap()
        )
    }
}
