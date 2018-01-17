table! {
    coins (id) {
        id -> Nullable<Integer>,
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
        published -> Bool,
    }
}
