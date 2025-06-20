{
  self,
  pname,
  nixpkgs,
  rust-overlay,
  crane,
}:
system:
let
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rust-overlay.overlays.default ];
  };

  inherit (pkgs.lib.trivial) flip;

  cranes = import ./cranes.nix { inherit self pkgs crane; };

  lib = {
    inherit
      self
      pname
      pkgs
      cranes
      ;

    import = flip import lib;

    build-workspace = lib.import ./build-workspace.nix;

    run-command =
      suffix: deps: script:
      let
        inherit (pkgs) runCommand;
        inherit (pkgs.lib) makeBinPath;

        fullScript = ''export PATH="$PATH:${makeBinPath deps}"'' + "\n" + script;
      in
      runCommand "${pname}-cmd-${suffix}" { } fullScript;
  };
in
lib
