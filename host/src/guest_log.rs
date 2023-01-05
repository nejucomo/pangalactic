pub(crate) fn bytes(b: &[u8]) {
    log::debug!("{}", String::from_utf8_lossy(b));
}
