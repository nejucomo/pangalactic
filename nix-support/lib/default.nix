{
  self,
  nixpkgs,
  rust-overlay,
  crane,
  system,
}:
let
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rust-overlay.overlays.default ];
  };

  rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile (self + "/rust-toolchain.toml");

  lib = {
    inherit self pkgs;

    import = path: import path lib;

    crane = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

    run-command =
      name-suffix: deps: script:
      let
        inherit (pkgs) runCommand;
        inherit (pkgs.lib) makeBinPath;
        name = "pangalactic-cmd-${name-suffix}";

        fullScript =
          ''
            export PATH="$PATH:${makeBinPath deps}"
          ''
          + "\n"
          + script;
      in
      pkgs.runCommand name { } fullScript;
  };
in
lib
