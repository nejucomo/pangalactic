use aliri_braid::{braid, Validator};

use crate::{Name, NameError, NameRef, NonEmptyPathRef, SEPARATOR};

#[braid(validator, serde)]
pub struct Path;

impl Path {
    pub fn from_utf8<B>(bytes: B) -> Result<Self, NameError>
    where
        B: AsRef<[u8]>,
    {
        crate::from_utf8(bytes)
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.take().into_bytes()
    }

    pub fn pop_last(&mut self) -> Option<Name> {
        if let Some(i) = self.0.rfind(SEPARATOR) {
            let mut suffix = self.0.split_off(i);
            let sep = suffix.remove(0);
            assert_eq!(sep, SEPARATOR);
            Some(unsafe { Name::new_unchecked(suffix) })
        } else {
            None
        }
    }
}

impl Validator for Path {
    type Error = NameError;

    fn validate(raw: &str) -> Result<(), NameError> {
        for name in split_components(raw) {
            <crate::Name as Validator>::validate(name)?;
        }
        Ok(())
    }
}

impl Default for Path {
    fn default() -> Self {
        Path("".to_string())
    }
}

impl PathRef {
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    pub fn components(&self) -> impl Iterator<Item = &NameRef> {
        split_components(&self.0).map(|n| NameRef::from_str(n).unwrap())
    }

    pub fn split_last(&self) -> Option<(&PathRef, &NameRef)> {
        let s = self.as_str();
        if s.is_empty() {
            None
        } else {
            Some(unsafe { NonEmptyPathRef::from_str_unchecked(s) }.split_last())
        }
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

fn split_components(s: &str) -> impl Iterator<Item = &str> {
    // Note: filtering out empty components should only filter out the single case where self.0 == "" which yields [""]
    s.split('/').filter(|s| !s.is_empty())
}
