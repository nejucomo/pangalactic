inputs: system:
let
  lib = import ./lib (inputs // { inherit system; });

  vendordir = lib.crane.vendorCargoDeps { inherit (lib) src; };
in
{
  packages = {
    default = lib.run-command "pkg-todo" [ ] ''
      echo 'Currently only the `...#book` output is implemented.'
      echo
      echo 'TO DO... implement `nix build`'

      mkdir "$out"
    '';

    inherit vendordir;

    book = lib.import ./book.nix { inherit vendordir; };
  };

  devShells.default = lib.import ./dev-shell.nix;
}
