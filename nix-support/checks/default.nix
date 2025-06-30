{ pkgs, run-command, ... }:
packages:
let
  inherit (builtins) attrNames readDir readFile;
  inherit (pkgs.lib.attrsets) filterAttrs mapAttrs mapAttrs';
  inherit (pkgs.lib.strings) hasSuffix removeSuffix;
  inherit (pkgs.lib.trivial) pipe;

  filterShellScripts = filterAttrs (n: k: k == "regular" && n != "framework.sh" && hasSuffix ".sh" n);

  mapAttrsToPaths = mapAttrs (n: _: ./. + "/${n}");

  buildChecks = mapAttrs' (
    fname: script: rec {
      name = removeSuffix ".sh" fname;
      value = run-command "check-${name}" [ packages.install ] ''
        exec '${./framework.sh}' '${script}'
      '';
    }
  );
in
pipe ./. [
  readDir
  filterShellScripts
  mapAttrsToPaths
  buildChecks
]
