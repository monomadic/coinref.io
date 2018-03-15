# coinref.io

run server:
```bash
cargo run --bin coinref
```

reset database:
```bash
rm database.sql
diesel setup --database-url ./database.sql
diesel print-schema --database-url ./database.sql > src/schema.rs
```

import data:
```bash
cargo run --bin coinref-import
```
