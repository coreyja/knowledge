#!/usr/bin/env bash

set -e

pushd $(git rev-parse --show-toplevel) > /dev/null

cargo install sqlx-cli --version 0.7.4  --no-default-features --features postgres,rustls
cargo sqlx database create
DATABASE_URL="${DATABASE_URL}_test" cargo sqlx database create

./scripts/migrate.sh

popd > /dev/null
