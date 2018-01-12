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

pub fn landing(coins: Vec<&str>) -> String {
    layout(
        format!("conref.io"),
        html! {
            a(href="/") { img(src="/static/logo.png", height="31px") }
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
                a(href="/") { img(src="/static/logo.png", height="31px") }
                h1 {: coin.name }
                img(src=format!("/static/icons/{}.png", coin.image));
                article {
                    h2 {: "Summary" }
                    p {: coin.summary }
                }
            }.into_string().unwrap()
        )
    }
}
