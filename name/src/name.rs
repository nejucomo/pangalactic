use aliri_braid::{braid, Validator};

use crate::{InvalidName, NameError};

pub const SEPARATOR: char = '/';

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
