lib:
{ cargoVendorDir }:
let
  inherit (lib) self run-command;
  inherit (lib.pkgs) fd mdbook graphviz;

  depgraph =
    run-command "depgraph"
      (with lib.pkgs; [
        cargo
        cargo-depgraph
        graphviz
      ])
      ''
        export CARGO_HOME='${cargoVendorDir}'
        export CARGO_NET_OFFLINE='true'
        mkdir "$out"
        cd "${self}"
        cargo depgraph \
          --frozen \
          --offline \
          --locked \
          --workspace-only \
          --dedup-transitive-deps \
          | dot \
            -Tsvg \
          > "$out/depgraph-ws-dedup.svg"
      '';

  booksrc = self + "/book";
in
run-command "render-book" [ fd mdbook ] ''
  mdbook build --dest-dir "$out" '${booksrc}'
  mkdir -p "$out/assets/generated"
  fd --glob '*.svg' '${depgraph}' --exec cp '{}' "$out/assets/generated/"
''
