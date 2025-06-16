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
  packages = {
    default = pkgs.runCommand "pangalactic-nix-pkg-todo" { } ''
      echo 'Currently only the `...#book` output is implemented.'
      echo
      echo 'TO DO... implement `nix build`'

      mkdir "$out"
    '';

    inherit book;
  };

  devShells.default = dev-shell;
}
