CREATE TABLE coins (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  symbol VARCHAR NOT NULL,
  website VARCHAR NOT NULL,
  twitter VARCHAR,
  reddit VARCHAR,
  github VARCHAR,
  telegram VARCHAR,
  slack VARCHAR,
  facebook VARCHAR,
  -- youtube VARCHAR,
  -- instagram VARCHAR,
  -- pinterest VARCHAR,
  -- discord VARCHAR,
  market_cap_usd REAL,
  market_cap_rank INTEGER,
  circulating_supply INTEGER,
  price_in_btc REAL,
  price_in_usd REAL,
  page TEXT NOT NULL DEFAULT ''
);

CREATE INDEX coin_symbol ON coins (symbol);
