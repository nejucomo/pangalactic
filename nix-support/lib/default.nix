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
      rawArgs@{
        src,
        cargoVendorDir,

        # Our own custom params:
        pnameSuffix,
        targetsRgx,
        manifestDir ? ".",
        ...
      }:
      let
        inherit (lib.crane) buildDepsOnly cargoBuild;
        inherit (lib) run-command;

        commonArgs =
          (removeAttrs rawArgs [
            "pnameSuffix"
            "targetsRgx"
            "manifestDir"
          ])
          // {
            pname = "${pname}-${pnameSuffix}";
            cargoExtraArgs = "--offline --target-dir=target/ --manifest-path ${manifestDir}/Cargo.toml";
          };

        cargoArtifacts = buildDepsOnly commonArgs;

        cargoBuilt = cargoBuild (
          commonArgs
          // {
            inherit cargoArtifacts;
            installCargoArtifactsMode = "use-symlink";
          }
        );

      in
      run-command "${pnameSuffix}-select-targets" [ pkgs.fd ] ''
        targetDir='${cargoBuilt}/target'
        echo 'Selecting "${targetsRgx}" from:' "$targetDir"
        mkdir "$out"
        fd '${targetsRgx}' "$targetDir" --full-path --exec ln -s '{}' "$out/"
      '';

    run-command =
      suffix: deps: script:
      let
        inherit (pkgs) runCommand;
        inherit (pkgs.lib) makeBinPath;

        fullScript = ''export PATH="$PATH:${makeBinPath deps}"'' + "\n" + script;
      in
      runCommand "${pname}-cmd-${suffix}" { } fullScript;
  };
in
lib
