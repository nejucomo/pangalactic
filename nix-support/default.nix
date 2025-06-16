src:
{
  self,
  nixpkgs,
  flake-utils,
  rust-overlay,
  crane,
}:
flake-utils.lib.eachDefaultSystem (
  import ./init-system.nix {
    inherit src;

    nixpkgs-flake = nixpkgs;
    rust-overlay-flake = rust-overlay;
    crane-flake = crane;
  }
)
