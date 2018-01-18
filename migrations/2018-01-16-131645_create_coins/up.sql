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
  market_cap INTEGER NOT NULL DEFAULT 0,
  page TEXT NOT NULL DEFAULT ''
);

CREATE INDEX coin_symbol ON coins (symbol);

INSERT INTO coins (name, symbol, website) VALUES ("Bitcoin", "BTC", "bitcoin.org");
INSERT INTO coins (name, symbol, website) VALUES ("Cobinhood", "COB", "cobinhood.com");
INSERT INTO coins (name, symbol, website) VALUES ("Eidoo", "EDO", "eidoo.io");
INSERT INTO coins (name, symbol, website, twitter) VALUES ("Etherium Classic", "ETC", "ethereumclassic.github.io", "eth_classic");
INSERT INTO coins (name, symbol, website, twitter) VALUES ("Raiblocks", "XRB", "raiblocks.net", "raiblocks");
