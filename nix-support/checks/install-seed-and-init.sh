log-run pg-install-seed --dirdb ./dirdb
log-run pg --dirdb ./dirdb init
set -x
[ -d .pg/parent.pglink ] || exit 1
