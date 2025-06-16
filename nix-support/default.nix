src:
{
  self,
  nixpkgs,
  flake-utils,
  rust-overlay,
}:
flake-utils.lib.eachDefaultSystem (import ./init-system.nix { inherit src nixpkgs rust-overlay; })
