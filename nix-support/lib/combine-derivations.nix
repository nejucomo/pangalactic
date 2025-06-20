{ pkgs, run-command, ... }:
derivs:
let
  inherit (pkgs.lib.attrsets) mapAttrsToList;
  inherit (pkgs.lib.strings) concatStringsSep;
  inherit (pkgs.lib.trivial) pipe;

  derivstr = pipe derivs [
    (mapAttrsToList (n: v: "${n}:${v}"))
    (concatStringsSep " ")
  ];
in
run-command "combine-derivations" [ pkgs.sd ] ''
  mkdir "$out"
  for nv in ${derivstr}
  do
    n="$(echo "$nv" | sd ':.*$' ''')"
    v="$(echo "$nv" | sd '^.*:' ''')"
    ln -sv "$v" "$out/$n"
  done
''
