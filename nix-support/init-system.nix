{
  src,
  nixpkgs,
  rust-overlay,
}:
system:
let
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rust-overlay.overlays.default ];
  };

  rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile (src + "/rust-toolchain.toml");

  book = import ./book.nix { inherit src pkgs; };

  dev-shell = import ./dev-shell.nix { inherit pkgs rust-toolchain; };
in
{
  packages = { inherit book; };

  devShells.default = dev-shell;
}
