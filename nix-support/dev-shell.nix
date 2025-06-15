{ mkShell, rust-toolchain }:
mkShell {
  packages = [ rust-toolchain ];
  shellHook = ''
    [ -n "$XDG_CONFIG_HOME" ] || XDG_CONFIG_HOME="$HOME/.config"

    NIX_DEVELOP_RC="$XDG_CONFIG_HOME/nix/develop.rc"

    [ -f "$NIX_DEVELOP_RC" ] && source "$NIX_DEVELOP_RC"

    export PATH="$(pwd)/target/debug:$PATH"
  '';
}
