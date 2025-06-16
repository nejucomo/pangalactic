use std::ffi::OsStr;

use aliri_braid::{braid, Validator};

use crate::{InvalidName, NameError, SEPARATOR};

#[braid(validator, serde)]
pub struct Name;

impl Name {
    pub fn from_utf8<B>(bytes: B) -> Result<Self, NameError>
    where
        B: AsRef<[u8]>,
    {
        use std::borrow::Cow::*;

        match String::from_utf8_lossy(bytes.as_ref()) {
            // It was good utf8:
            Borrowed(s) => Name::try_from(s),
            // It was bad utf8:
            Owned(input) => Err(NameError {
                input,
                reason: InvalidName::UTF8,
            }),
        }
    }

    pub fn from_os_str<S>(s: S) -> Result<Self, NameError>
    where
        S: AsRef<OsStr>,
    {
        Self::from_utf8(s.as_ref().as_encoded_bytes())
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.take().into_bytes()
    }
}

impl Validator for Name {
    type Error = NameError;

    fn validate(raw: &str) -> Result<(), NameError> {
        fn validate_inner(raw: &str) -> Result<(), InvalidName> {
            use InvalidName::*;
            if raw.is_empty() {
                Err(Empty)
            } else if raw.find(SEPARATOR).is_some() {
                Err(ContainsSeparator)
            } else {
                Ok(())
            }
        }

        validate_inner(raw).map_err(|reason| NameError {
            input: raw.to_string(),
            reason,
        })
    }
}

impl AsRef<std::path::Path> for Name {
    fn as_ref(&self) -> &std::path::Path {
        let s: &str = self.as_ref();
        s.as_ref()
    }
}

impl NameRef {
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl AsRef<NameRef> for NameRef {
    fn as_ref(&self) -> &NameRef {
        self
    }
}

impl<'a> From<&'a Name> for &'a NameRef {
    fn from(name: &'a Name) -> Self {
        name.as_ref()
    }
}
