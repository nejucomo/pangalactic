use aliri_braid::{braid, Validator};

use crate::{InvalidPath, NameRef, PathError, SEPARATOR};

#[braid(validator, serde)]
pub struct Path;

impl Path {
    pub fn from_utf8<B>(bytes: B) -> Result<Self, PathError>
    where
        B: AsRef<[u8]>,
    {
        use std::borrow::Cow::*;

        match String::from_utf8_lossy(bytes.as_ref()) {
            // It was good utf8:
            Borrowed(s) => Path::try_from(s),
            // It was bad utf8:
            Owned(input) => Err(PathError {
                input,
                reason: InvalidPath::UTF8,
            }),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.take().into_bytes()
    }
}

impl Validator for Path {
    type Error = PathError;

    fn validate(raw: &str) -> Result<(), PathError> {
        fn validate_inner(raw: &str) -> Result<(), InvalidPath> {
            use InvalidPath::*;

            if raw.is_empty() {
                Err(Empty)
            } else {
                for name in raw.split(SEPARATOR) {
                    <crate::Name as Validator>::validate(name).map_err(Name)?;
                }
                Ok(())
            }
        }

        validate_inner(raw).map_err(|reason| PathError {
            input: raw.to_string(),
            reason,
        })
    }
}

impl PathRef {
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    pub fn components(&self) -> impl Iterator<Item = &NameRef> {
        self.0
            .split('/')
            .map(|n| unsafe { NameRef::from_str_unchecked(n) })
    }
}

impl AsRef<PathRef> for PathRef {
    fn as_ref(&self) -> &PathRef {
        self
    }
}

impl<'a> From<&'a Path> for &'a PathRef {
    fn from(name: &'a Path) -> Self {
        name.as_ref()
    }
}
