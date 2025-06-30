#! /usr/bin/env bash
set -eu

function fail
{
  echo "FAIL: $*"
  exit 1
}

[ $# -eq 1 ] || fail 'expected single <check-script> argument'

function log-run
{
  echo "Running: $*"
  eval "$@"
}

mkdir "$out"

set +e
(
  set -ex
  source "$1"
) 2> "$out/stderr.log"

if [ $? = 0 ]
then
  echo "ok: check $1"
else
  fail "check $1"
fi
