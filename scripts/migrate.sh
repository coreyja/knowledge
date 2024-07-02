#!/usr/bin/env bash

set -e

pushd $(git rev-parse --show-toplevel) > /dev/null

cargo sqlx migrate run --source cores/migrations

popd > /dev/null
