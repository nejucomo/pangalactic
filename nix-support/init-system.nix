{
  rust-toolchain-toml,
  nixpkgs,
  rust-overlay,
}:
system:
let
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rust-overlay.overlays.default ];
  };

  rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile rust-toolchain-toml;
in
{
  devShells.default = import ./dev-shell.nix { inherit pkgs rust-toolchain; };
}
