{
  src,
  nixpkgs-flake,
  rust-overlay-flake,
  crane-flake,
  system,
}:
let
  pkgs = import nixpkgs-flake {
    inherit system;
    overlays = [ rust-overlay-flake.overlays.default ];
  };

  rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile (src + "/rust-toolchain.toml");

  lib = {
    inherit src pkgs;

    import = path: import path lib;

    crane = (crane-flake.mkLib pkgs).overrideToolchain rust-toolchain;

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
