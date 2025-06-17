inputs@{ flake-utils, ... }:
let
  inherit (flake-utils.lib) eachDefaultSystem;

  sysInputs = removeAttrs inputs [ "flake-utils" ];
in
eachDefaultSystem (import ./init-system.nix sysInputs)
