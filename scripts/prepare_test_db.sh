#!/usr/bin/env bash

set -e

pushd $(git rev-parse --show-toplevel) > /dev/null

export DATABASE_URL="${DATABASE_URL}_test"

cargo sqlx database drop -y
cargo sqlx database create
