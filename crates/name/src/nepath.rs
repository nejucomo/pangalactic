use std::ops::Deref;

use aliri_braid::{braid, Validator};

use crate::{InvalidName, Name, NameError, NameRef, Path, PathRef, SEPARATOR};

#[braid(validator, serde)]
pub struct NonEmptyPath;

impl NonEmptyPath {
    pub fn from_utf8<B>(bytes: B) -> Result<Self, NameError>
    where
        B: AsRef<[u8]>,
    {
        crate::from_utf8(bytes)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.take().into_bytes()
    }

    pub fn split_last(self) -> (Path, Name) {
        let mut path = Path::from(self);
        let name = path.pop_last().unwrap();
        (path, name)
    }
}

impl Validator for NonEmptyPath {
    type Error = NameError;

    fn validate(raw: &str) -> Result<(), NameError> {
        {
            use InvalidName::*;

            if raw.is_empty() {
                Err(NameError {
                    input: raw.to_string(),
                    reason: Empty,
                })
            } else {
                <Path as Validator>::validate(raw)
            }
        }
    }
}

impl From<NonEmptyPath> for Path {
    fn from(ne: NonEmptyPath) -> Self {
        unsafe { Path::new_unchecked(ne.0) }
    }
}

impl TryFrom<Path> for NonEmptyPath {
    type Error = NameError;

    fn try_from(p: Path) -> Result<Self, Self::Error> {
        if p.is_empty() {
            Err(NameError {
                input: p.into(),
                reason: InvalidName::Empty,
            })
        } else {
            Ok(unsafe { NonEmptyPath::new_unchecked(p.into()) })
        }
    }
}

impl NonEmptyPathRef {
    pub fn split_last(&self) -> (&PathRef, &NameRef) {
        if let Some((prefix, suffix)) = self.as_str().rsplit_once(SEPARATOR) {
            let prefix = unsafe { PathRef::from_str_unchecked(prefix) };
            let suffix = unsafe { NameRef::from_str_unchecked(suffix) };
            (prefix, suffix)
        } else {
            let prefix = unsafe { PathRef::from_str_unchecked("") };
            let suffix = NameRef::from_str(self.as_str()).unwrap();
            (prefix, suffix)
        }
    }
}

impl Deref for NonEmptyPathRef {
    type Target = PathRef;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a> From<&'a NonEmptyPath> for &'a NonEmptyPathRef {
    fn from(name: &'a NonEmptyPath) -> Self {
        name.as_ref()
    }
}

impl AsRef<NonEmptyPathRef> for NonEmptyPathRef {
    fn as_ref(&self) -> &NonEmptyPathRef {
        self
    }
}

impl AsRef<PathRef> for NonEmptyPathRef {
    fn as_ref(&self) -> &PathRef {
        unsafe { PathRef::from_str_unchecked(self.as_ref()) }
    }
}
