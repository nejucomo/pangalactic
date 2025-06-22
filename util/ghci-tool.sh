#! /usr/bin/env bash
#
# Utilities to make github actions less tedious.

HOOKS_DIR="./ghci-tool-hooks"

function main
{
  case "$*" in
    push-exit-hook|run-exit-hooks)
      eval "$@"
      ;;
    *)
      echo "Unexpected args: $*"
      exit 1
  esac
}

# Push stdin into an exit hook script
function push-exit-hook
{
  cat > "$(alloc-exit-hook)"
}

function run-exit-hooks
{
  for ix in $(ls "$HOOKS_DIR" | sort -rn)
  do
    local hook="$HOOKS_DIR/$ix"
    chmod u+x hook
    echo "Executing cleanup hook: $hook"
    eval "$hook"
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
