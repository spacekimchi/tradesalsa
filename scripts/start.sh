#! /usr/bin/env bash

./scripts/init_db.sh

./scripts/init_redis.sh

# cargo watch -x run
# --no-vcs-ignores tells watch to ignore filtering out files in version control systems (.gitignore)
cargo watch --no-vcs-ignores -x run
