lib:
{ vendordir }:
let
  inherit (lib) src run-command;
  inherit (lib.pkgs) mdbook graphviz;

  depgraph =
    run-command "depgraph"
      (with lib.pkgs; [
        cargo
        cargo-depgraph
        graphviz
      ])
      ''
        ( # subshell for xtrace scoping
        set -x
        export CARGO_HOME='${vendordir}'
        export CARGO_NET_OFFLINE='true'
        mkdir "$out"
        cd "${src}"
        cargo depgraph \
          --frozen \
          --offline \
          --locked \
          --workspace-only \
          --dedup-transitive-deps \
          | dot \
            -Tsvg \
          > "$out/depgraph-ws-dedup.svg"
        )
      '';

  booksrc = src + "/book";
in
run-command "render-book" [ mdbook ] ''
  mdbook build --dest-dir "$out" '${booksrc}'
  mkdir -p "$out/assets/generated"
  cp -rv '${depgraph}/.' "$out/assets/generated/"
''
