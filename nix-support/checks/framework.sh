#! /usr/bin/env bash
set -eu

function fail
{
  echo "FAIL: $*"
  exit 1
}

function log-run
{
  echo "Running: $*"
  eval "$@"
}

function check-test
{
  [ $# -eq 2 ] || fail "(internal) check-test called with $# args instead of 2"

  local testop="$1"
  local path="$2"

  [ "$testop" "$path" ] || fail "[ $testop $path ] did not succeed."
}

[ $# -eq 1 ] || fail 'expected single <check-script> argument'

mkdir "$out"
logpath="$out/stderr.log"

set +e
(
  set -ex
  source "$1"
) 2> "$logpath"

if [ $? = 0 ]
then
  echo "ok: check $1"
else
  echo "+-= start: $logpath"
  sed 's/^/|  /' "$logpath"
  echo "+-= end: $logpath"
  echo
  fail "check $1"
fi
