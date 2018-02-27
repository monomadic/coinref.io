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
  market_cap INTEGER NOT NULL DEFAULT 0,
  page TEXT NOT NULL DEFAULT ''
);

CREATE INDEX coin_symbol ON coins (symbol);
