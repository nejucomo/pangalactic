#!/usr/bin/env bash
set -euo pipefail

function main
{
  if [ $# -eq 0 ]
  then
    usage-error 'no command provided'
  fi

  cmd="$1" ; shift
  
  case "$cmd" in
    outpath|restore|refresh)
      eval "$cmd" "$@"
      ;;

    *)
      usage-error "unknown command \"$cmd\""
      ;;
  esac
}

function usage-error
{
  cat <<__EOF
    Usage Error: $*

    Usage:

    $0 outpath <flake-output>
      Print the derivation path for <flake-output>. This is suitable as
      a caching key.

    $0 restore <flake-output> <cache-dir>
      Load <cache-dir> into the nix store, if present.      

    $0 refresh <flake-output> <cache-dir> <outpath>
      If <cache-dir> is absent or if the outpath of <flake-output>
      differs from <outpath> (which indicates a stale cache),
      then export the closure of <flake-output> to <cache-dir>.
__EOF

  exit 1
}

function usage-check-arity
{
  expected="$1" ; shift
  [ $# -eq "$expected" ] || usage-error "expected ${expected} args, got $#: $*"
}

function outpath
{
  usage-check-arity 1 "$@"
  local flake_name="$1"

  nix eval --raw ".#$flake_name"
}

function restore
{
  usage-check-arity 2 "$@"
  local flake_name="$1"
  local cache_dir="$2"
  local out_path="$(outpath "$flake_name")"

  if [ -d "$cache_dir" ]
  then
    if nix copy --no-check-sigs --from "file://${cache_dir}" "$out_path"; then
      echo "✅ Successfully restored .#${flake_name} from cache."
    else
      echo '🟢 Cache miss...'
    fi
  else
      echo "🟡 No cache directory, new build: ${cache_dir}"
  fi

  exec nix build ".#${flake_name}"
}

function refresh
{
  usage-check-arity 3 "$@"
  local flake_name="$1"
  local cache_dir="$2"
  local expected_outpath="$3"

  if needs-refresh "$flake_name" "$cache_dir" "$expected_outpath"
  then
    echo '⏳ Refreshing cache...'
    nix build ".#${flake_name}"

    mkdir -p "$cache_dir"
    nix copy --to "file://${cache_dir}" ".#${flake_name}"
  else
    echo '✅ Cache up-to-date.'
  fi
}

function needs-refresh
{
  local flake_name="$1"
  local cache_dir="$2"
  local expected_outpath="$3"

  if ! [ -d "$cache_dir" ]
  then
    echo "No cache dir found at $cache_dir"
    return 0
  else
    local actual_outpath="$(outpath "$flake_name")"
    if ! [ "$actual_outpath" = "$expected_outpath" ]
    then
      echo "Stale cache at: $cache_dir"
      echo "-expected derivation path: $expected_outpath"
      echo "-  actual derivation path: $actual_outpath"
      return 0
    else
      return 1
    fi
  fi
}

main "$@"
