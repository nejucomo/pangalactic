{ pkgs, run-command, ... }:
name: derivs:
let
  inherit (pkgs.lib.attrsets) mapAttrsToList;
  inherit (pkgs.lib.strings) concatStringsSep;
  inherit (pkgs.lib.trivial) pipe;

  derivstr = pipe derivs [
    (mapAttrsToList (n: v: "${n}:${v}"))
    (concatStringsSep " ")
  ];
in
run-command "combine-derivations-${name}" [ pkgs.sd ] ''
  mkdir "$out"
  for item in ${derivstr}
  do
    link="$(echo "$item" | sd ':.*$' ''')"
    target="$(echo "$item" | sd '^.*:' ''')"
    mkdir -p "$(dirname "$out/$link")"
    ln -sv "$target" "$out/$link"
  done
''
