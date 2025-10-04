#!/usr/bin/env bash

SCRIPT=$(readlink -f "$0")
SCRIPT_PATH=$(dirname "$SCRIPT")

(cd $SCRIPT_PATH && cargo fmt && cargo clippy)
