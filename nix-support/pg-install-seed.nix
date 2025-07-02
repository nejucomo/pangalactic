{ pkgs, ... }:
{ bin, seed-dir }:

let
  name = "pg-install-seed";
in
pkgs.writeTextFile {
  inherit name;

  executable = true;
  destination = "/${name}";
  text = ''
    function usage {
      cat <<__EOF
      error: $*

      usage: $0 [ --dirdb <dirdb> ]

        Install the seed into the store; print its link on stdout.
    __EOF

      exit 1
    }

    if [ $# -eq 0 ]
    then
      dirdbOpts=""
    else
      [ "$1" = '--dirdb' ] || usage "unknown option: $1"
      [ $# -gt 1 ] || usage 'missing `--dirdb <dirdb>` argument'
      [ $# -eq 2 ] || usage "unexpected arguments: $*"

      dirdbOpts="--dirdb $2"
    fi

    '${bin.outputs}/pg-store' $dirdbOpts xfer '${seed-dir}' 'pg:'
  '';
}
