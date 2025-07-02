{
  pname,
  self,
  run-command,
  pkgs,
  ...
}:
{ cargoVendorDir }:
let
  inherit (pkgs) fd mdbook graphviz;

  depgraph =
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
      '';

  booksrc = self + "/book";
in
run-command "render-book" [ fd mdbook ] ''
  mdbook build --dest-dir "$out" '${booksrc}'
  mkdir -p "$out/assets/generated"
  fd --glob '*.svg' '${depgraph}' --exec cp '{}' "$out/assets/generated/"
''
