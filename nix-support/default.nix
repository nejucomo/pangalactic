inputs@{ flake-utils, ... }:
{ pname }:
let
  inherit (flake-utils.lib) eachDefaultSystem;

  sysInputs = removeAttrs inputs [ "flake-utils" ] // {
    inherit pname;
  };
in
eachDefaultSystem (import ./init-system.nix sysInputs)
