CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL
);

CREATE TABLE coin_tags (
  coin_id INTEGER NOT NULL, 
  tag_id INTEGER NOT NULL, 
  PRIMARY KEY (coin_id, tag_id), 
  FOREIGN KEY (coin_id) REFERENCES coins (id) ON DELETE CASCADE, 
  FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- CREATE UNIQUE INDEX unique_revision_per_article_id ON article_revisions
--     (article_id, revision);

INSERT INTO tags (name) VALUES ("transactional");
INSERT INTO tags (name) VALUES ("decentralised");
INSERT INTO tags (name) VALUES ("semi centralised");
INSERT INTO tags (name) VALUES ("centralised");
INSERT INTO tags (name) VALUES ("content distribution");
INSERT INTO tags (name) VALUES ("b2b tools");
INSERT INTO tags (name) VALUES ("ultrafast transactions");
INSERT INTO tags (name) VALUES ("privacycoin");
INSERT INTO tags (name) VALUES ("shitcoin");

INSERT INTO tags (name) VALUES ("zero fee");
INSERT INTO coin_tags (coin_id, tag_id) VALUES ((SELECT id FROM coins WHERE symbol="XRB"), (SELECT id FROM tags WHERE name="ultrafast transactions"));
