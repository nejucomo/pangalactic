{
  pname,
  self,
  run-command,
  pkgs,
  ...
}:
{ depgraphs }:
let
  inherit (pkgs) fd mdbook;

  booksrc = self + "/book";
in
run-command "render-book" [ fd mdbook ] ''
  mdbook build --dest-dir "$out" '${booksrc}'
  mkdir -p "$out/assets/generated"
  fd --glob '*.svg' '${depgraphs}' --exec cp '{}' "$out/assets/generated/"
''
