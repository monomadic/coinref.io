CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL
);

CREATE TABLE coin_tags (
  CoinId INTEGER NOT NULL, 
  TagId INTEGER NOT NULL, 
  PRIMARY KEY (CoinId, TagId), 
  FOREIGN KEY (CoinId) REFERENCES Coins (ID) ON DELETE CASCADE, 
  FOREIGN KEY (TagId) REFERENCES Tags (ID) ON DELETE CASCADE
);

-- CREATE UNIQUE INDEX unique_revision_per_article_id ON article_revisions
--     (article_id, revision);
