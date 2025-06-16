src:
{
  self,
  nixpkgs-flake,
  flake-utils-flake,
  rust-overlay-flake,
  crane-flake,
}:
flake-utils-flake.lib.eachDefaultSystem (
  import ./init-system.nix {
    inherit
      src
      nixpkgs-flake
      rust-overlay-flake
      crane-flake
      ;
  }
)
