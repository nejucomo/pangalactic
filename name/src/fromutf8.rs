use crate::{InvalidName, NameError};

pub(crate) fn from_utf8<B, T>(bytes: B) -> Result<T, NameError>
where
    B: AsRef<[u8]>,
    T: for<'a> TryFrom<&'a str, Error = NameError>,
{
    use std::borrow::Cow::*;

    match String::from_utf8_lossy(bytes.as_ref()) {
        // It was good utf8:
        Borrowed(s) => T::try_from(s),
        // It was bad utf8:
        Owned(input) => Err(NameError {
            input,
            reason: InvalidName::UTF8,
        }),
    }
}
