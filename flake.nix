{
  description = "Pangalactic deterministic computation on the universal directed acyclic graph";

  inputs = {
    nixpkgs-flake.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils-flake.url = "github:numtide/flake-utils";
    rust-overlay-flake.url = "github:oxalica/rust-overlay";
    crane-flake.url = "github:ipetkov/crane";
  };

  outputs = inputs: import ./nix-support ./. inputs;
}
