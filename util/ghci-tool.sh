#! /usr/bin/env bash
#
# Utilities to make github actions less tedious.

HOOKS_DIR="./ghci-tool-hooks"

function main
{
  case "$1" in
    run-exit-hooks | push-exit-hook)
      eval "$@"
      ;;
    *)
      echo "Unknown command: $1"
      exit 1
  esac
}

# Push stdin into an exit hook script
function push-exit-hook
{
  local hook="$(alloc-exit-hook)"
  echo "Pushing $hook: $*"
  echo "$*" > "$hook"
}

function run-exit-hooks
{
  if [ $# -gt 0 ]
  then
    echo "Unexpected args: $*"
    exit 1
  fi

  for ix in $(ls "$HOOKS_DIR" | sort -rn)
  do
    local hook="$HOOKS_DIR/$ix"
    echo "Executing cleanup hook $hook: $(cat "$hook")"
    bash "$hook"
  done
}

function alloc-exit-hook
{
  mkdir -p "$HOOKS_DIR"

  for i in $(seq 0 999)
  do
    local candidate="$HOOKS_DIR/$i"
    if ! [ -e "$candidate" ]
    then
      echo "$candidate"
      return
    fi
  done

  echo 'ran out of candidate paths; something is wrong' >&2
  exit 1
}

main "$@"
