{
  description = "Rust dev shell with native and wasm32-unknown-unknown support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ rustToolchain ];
          shellHook = ''
            [ -n "$XDG_CONFIG_HOME" ] || XDG_CONFIG_HOME="$HOME/.config"

            NIX_DEVELOP_RC="$XDG_CONFIG_HOME/nix/develop.rc"

            [ -f "$NIX_DEVELOP_RC" ] && source "$NIX_DEVELOP_RC"

            export PATH="$(pwd)/target/debug:$PATH"
          '';
        };
      }
    );
}
