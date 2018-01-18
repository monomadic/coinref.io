#!/bin/sh
rm database.sql
diesel setup --database-url ./database.sql
diesel print-schema > src/schema.rs --database-url ./database.sql
