{ pkgs, run-command, ... }:
packages:
let
  inherit (builtins) attrNames readDir readFile;
  inherit (pkgs.lib.attrsets) filterAttrs mapAttrs mapAttrs';
  inherit (pkgs.lib.strings) hasSuffix removeSuffix;
  inherit (pkgs.lib.trivial) pipe;

  filterShellScripts = filterAttrs (n: k: k == "regular" && hasSuffix ".sh" n);

  readScripts = mapAttrs (n: _: readFile (./. + "/${n}"));

  buildChecks = mapAttrs' (
    fname: script: rec {
      name = removeSuffix ".sh" fname;
      value = run-command "check-${name}" [ packages.install ] script;
    }
  );
in
pipe ./. [
  readDir
  filterShellScripts
  readScripts
  buildChecks
]
