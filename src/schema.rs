table! {
    coin_tags (CoinId, TagId) {
        CoinId -> Integer,
        TagId -> Integer,
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
        market_cap -> Integer,
        page -> Text,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    coin_tags,
    coins,
    tags,
);
