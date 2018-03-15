table! {
    coin_tags (coin_id, tag_id) {
        coin_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    coins (id) {
        id -> Integer,
        name -> Text,
        symbol -> Text,
        website -> Text,
        twitter -> Nullable<Text>,
        reddit -> Nullable<Text>,
        github -> Nullable<Text>,
        telegram -> Nullable<Text>,
        slack -> Nullable<Text>,
        facebook -> Nullable<Text>,
        market_cap_usd -> Nullable<Float>,
        market_cap_rank -> Nullable<Integer>,
        circulating_supply -> Nullable<Integer>,
        price_in_btc -> Nullable<Float>,
        price_in_usd -> Nullable<Float>,
        page -> Text,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(coin_tags -> coins (coin_id));
joinable!(coin_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    coin_tags,
    coins,
    tags,
);
