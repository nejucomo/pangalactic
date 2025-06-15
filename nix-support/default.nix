{
  self,
  nixpkgs,
  flake-utils,
  rust-overlay,
}:
rust-toolchain-toml:
flake-utils.lib.eachDefaultSystem (
  import ./init-system.nix { inherit rust-toolchain-toml nixpkgs rust-overlay; }
)
