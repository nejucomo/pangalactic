#!/usr/bin/env bash
set -euo pipefail

# Usage: ./restore-or-build.sh <flake-output-name> <cache-dir>
# Example: ./restore-or-build.sh vendor-cargo-deps /tmp/nix-copy-cache

OUTPUT_NAME="${1:-}"
CACHE_DIR="${2:-1}"

if [[ -z "$OUTPUT_NAME" ]] || [[ -z "$CACHE-DIR" ]]; then
  echo "❌ Usage: $0 <flake-output-name> <cache-dir>"
  exit 1
fi

echo "⏎ Attempting to restore: .#${OUTPUT_NAME} from ${CACHE_DIR}"

# Try to resolve the output path (this does not build)
OUT_PATH=$(nix eval --raw ".#${OUTPUT_NAME}.outPath")

# Try to copy it from the cache
if nix copy --from "file://${CACHE_DIR}" "$OUT_PATH"; then
  echo "✅ Successfully restored ${OUTPUT_NAME} from cache."
else
  echo "⚠️  Cache miss or invalid. Building ${OUTPUT_NAME}..."
  nix build ".#${OUTPUT_NAME}"
fi

# Verify the result now exists
if [[ -e "$OUT_PATH" ]]; then
  echo "✅ Path exists: $OUT_PATH"
else
  echo "❌ Failed to restore or build ${OUTPUT_NAME}"
  exit 1
fi
