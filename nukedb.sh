#!/bin/sh
rm database.sql
rm src/schema.rs
diesel setup --database-url ./database.sql
diesel print-schema --database-url ./database.sql > src/schema.rs
