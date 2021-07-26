#!/bin/bash
# This command automatically refreshes code coverage when changes are made.
CARGO_TARGET_DIR="./coverage_target" cargo watch -d 2 -x 'tarpaulin --features ws --ignore-tests --out Lcov' -i lcov.info -i coverage_target
