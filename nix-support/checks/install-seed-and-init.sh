log-run pg-install-seed --dirdb ./dirdb
log-run pg --dirdb ./dirdb init
[ -d .pg ] || exit 1
