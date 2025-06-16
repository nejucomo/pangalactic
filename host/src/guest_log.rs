pub(crate) fn bytes(b: &[u8]) {
    let guest_message = String::from_utf8_lossy(b);
    tracing::debug!(?guest_message);
}
