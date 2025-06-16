{ src, pkgs }:
let
  inherit (pkgs) runCommand mdbook;

  booksrc = src + "/book";
in
runCommand "pangalactic-render-book" { inherit mdbook; } ''
  '${mdbook}/bin/mdbook' build --dest-dir "$out" '${booksrc}'
''
