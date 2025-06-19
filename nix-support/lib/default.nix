{
  self,
  pname,
  nixpkgs,
  rust-overlay,
  crane,
}:
system:
let
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rust-overlay.overlays.default ];
  };

  rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile (self + "/rust-toolchain.toml");

  lib = {
    inherit
      self
      pname
      pkgs
      rust-toolchain
      ;

    import = path: import path lib;

    crane = (crane.mkLib pkgs).overrideToolchain rust-toolchain;

    build-workspace =
      wsargs@{
        pname,
        src,
        cargoVendorDir,
        relpath ? ".",
        ...
      }:
      let
        inherit (lib.crane) buildDepsOnly cargoBuild;

        commonArgs = (removeAttrs wsargs [ "relpath" ]) // {
          cargoExtraArgs = "--offline --target-dir=target/ --manifest-path ${relpath}/Cargo.toml";
        };

        inherit (pkgs.lib) trace;

        traceToString = v: trace "${v}" v;

        traceAttrNames = attrs: trace (builtins.attrNames attrs) attrs;

        cargoArtifacts = buildDepsOnly commonArgs;
      in
      cargoBuild (commonArgs // { inherit cargoArtifacts; });

    select-targets =
      targetsTarballDir: glob:
      lib.run-command "select-${glob}" [ pkgs.zstd ] ''
        echo 'Selecting "${glob}" from "${targetsTarballDir}"'
        mkdir "$out"
        tar -xf '${targetsTarballDir}/target.tar.zst'
        for rdir in $(find . -type d -name 'release')
        do
          find "$rdir" -maxdepth 1 -name '${glob}' -exec mv '{}' "$out/" ';'
        done
      '';

    run-command =
      suffix: deps: script:
      let
        inherit (pkgs) runCommand;
        inherit (pkgs.lib) makeBinPath;
        inherit (builtins) replaceStrings;

        esc-suffix = replaceStrings [ "." "*" ] [ "DOT" "STAR" ] suffix;

        name = "${pname}-cmd-${esc-suffix}";

        fullScript =
          ''
            export PATH="$PATH:${makeBinPath deps}"
          ''
          + "\n"
          + script;
      in
      pkgs.runCommand name { } fullScript;
  };
in
lib
