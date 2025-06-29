#! /usr/bin/env bash
set -eu

( set -x

  pg-install-seed --dirdb ./dirdb
  pg --dirdb ./dirdb init
  find . -exec ls -ld '{}' \;
)
