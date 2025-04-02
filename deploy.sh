#!/bin/bash

echo "Building Earn Vault..."
cargo build --release

echo "Running database migrations..."
sqlite3 earn_vault.db ".read migrations.sql"

echo "Deploying to Railway..."
railway up