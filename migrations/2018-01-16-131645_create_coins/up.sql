CREATE TABLE coins (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  symbol VARCHAR NOT NULL,
  website VARCHAR NOT NULL,
  twitter VARCHAR,
  reddit VARCHAR,
  github VARCHAR,
  telegram VARCHAR,
  slack VARCHAR,
  facebook VARCHAR,
  market_cap INTEGER,
  page TEXT NOT NULL
);
CREATE INDEX coin_symbol ON coins (symbol);

INSERT INTO coins (name, symbol, website, page)
VALUES ("Bitcoin", "BTC", "https://bitcoin.org", "");
