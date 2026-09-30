{
  pname,
  self,
  run-command,
  pkgs,
  ...
}:
{ cargoVendorDir }:
run-command "depgraph"
  (with pkgs; [
    cargo
    cargo-depgraph
    graphviz
  ])
  ''
    export CARGO_HOME='${cargoVendorDir}'
    export CARGO_NET_OFFLINE='true'

    function depgraph {
      local suffix="$1"
      shift

      cargo depgraph \
        --frozen \
        --workspace-only \
        --dedup-transitive-deps \
        "$@" \
        | dot \
          -Tsvg \
        > "$out/depgraph-$suffix.svg"
    }

    mkdir "$out"
    cd "${self}"

    depgraph 'host' \
      --exclude '${pname}' \
      --exclude '${pname}-guest' \
      --exclude '${pname}-test-runner' \
      --exclude '${pname}-test-dir'

    depgraph 'guest' \
      --root '${pname}-guest'

    depgraph 'all-deps' \
      --all-deps
  ''
