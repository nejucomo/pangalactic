{
  description = "Pangalactic deterministic computation on the universal directed acyclic graph";

  nixConfig = {
    substituter = [
      "https://cache.nixos.org"
      "https://nejucomo-pangalactic.cachix.org"
    ];

    trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "nejucomo-pangalactic.cachix.org-1:XEqcfPpgIDF21+PYI2lXb695rT761pHQs6bxye/dv+4="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs: import ./nix-support inputs { pname = "pangalactic"; };
}
