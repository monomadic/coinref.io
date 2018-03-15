#!/bin/sh
rm database.sql
diesel setup --database-url ./database.sql
diesel print-schema --database-url ./database.sql > src/schema.rs
