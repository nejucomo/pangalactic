{ run-command, ... }:
{ wasm }:
run-command "seed-dir" [ ] ''
  outbin="$out/bin"
  outbintest="$out/bin/test"
  mkdir -p "$outbintest"
  for wasm in '${wasm.outputs}'/*
  do
    outrel="$(basename "$wasm" | sed 's|\.wasm$||; s|^test_|test/|')"
    ln -sv "$wasm" "$outbin/$outrel"
  done
''
