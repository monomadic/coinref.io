SQLite format 3   @        	                                                             .�� 
� ���y 
��                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          ��wtablecoin_tagscoin_tagsCREATE TABLE coin_tags (
  coin_id INTEGER NOT NULL, 
  tag_id INTEGER NOT NULL, 
  PRIMARY KEY (coin_id, tag_id), 
  FOREIGN KEY (coin_id) REFERENCES coins (id) ON DELETE CASCADE, 
  FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
)1E indexsqlite_autoindex_coin_tags_1coin_tags	w�QtabletagstagsCREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL
)F#aindexcoin_symbolcoinsCREATE INDEX coin_symbol ON coins (symbol)P++Ytablesqlite_sequencesqlite_sequenceCREATE TABLE sqlite_sequence(name,seq)�C�etablecoinscoinsCREATE TABLE coins (
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
)�GAA�table__diesel_schema_migrations__diesel_schema_migrationsCREATE TABLE __diesel_schema_migrations (version VARCHAR(50) PRIMARY KEY NOT NULL,run_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP)SgA indexsqlite_autoindex___diesel_schema_migrations_1__diesel_schema_migrations          � ��                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        $)3201801170218012018-02-27 12:50:32$)3201801161316452018-02-27 12:50:32
   � ��                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               )20180117021801)	20180116131645   � ��:��V��                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   BottosBTO

�u 1      �%BancorBNTwww.bancor.network

p Bancor is a platform for smart tokens. Users sell tokens into the Bancor network, which can then be exchanged for other tokens without having to match buyers and sellers through an order book, like a traditional exchange. Instead, Bancor attempts to use an algorithm to determine a common price exchange between all tokens.

p Tokens are exchanged on-chain (directly on the blockchain) - which means there is no counterparty risk (no central authority controlling your coins).

https://steemit.com/bancor/@leopdspots/is-bancor-the-fractional-reserve-banking-system-of-crytocurrencies
* 1      CardanoADAwww.bancor.network
       EOSEOS

� #      �BitcoinBTCbitcoin.org

p The world's first decentralised digital currency.

ul
    li.pro extremely well established, the current market leader
L !      qWaltonCoinWTC

p Chinese run RFID solution. Competitor to VEN.
�!       �%IconICX

ul
    li verification network on a blockchain
    li south koreas first ICO - the NEO of south korea
    li working product, actively used (by who?)
    li open source
    li competes with ETH, NEO, QTUM, who are well established, large players (but more generalised)

h2 Pros
ul
    li.pro already established, working product
    li.pro extremely large scale team with esteemed credentials (separate blockchain, ai team, marketing team)
    li.pro founder, Min Kim, very up front and clear in interviews about objectives and technical issues.

h2 Cons
ul
    li.con centralised at the start
    li.con mainnet was pushed back (released now)
    li.con have not fully communicated their staking positions (why hold it?)

h2 Links
p https://www.youtube.com/watch?v=tCWYquJIEoE
R       �IOTAIOTA

p Currently centralised, plans to decentralise. Zero fee.
   � ��                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               	coinstags

   � ��������                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        BTOBNTADAEOSBTCWTCICX	IOTA   
K ������ueXK                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               
 zero fee	 shitcoin #privacycoin 9ultrafast transactions b2b tools 5content distribution #centralised -semi centralised 'decentralised 'transactional                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              