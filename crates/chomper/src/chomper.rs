use anyhow::{anyhow, bail, Result};
use std::{cell::Cell, fmt::Debug};

#[derive(Debug)]
pub struct Chomper<'a>(Cell<&'a str>);

impl<'a> Chomper<'a> {
    pub fn chomp_prefix<S>(&mut self, separator: S) -> Result<&'a str>
    where
        S: AsRef<str>,
    {
        let separator = separator.as_ref();
        let orig = self.0.get();
        self.try_chomp_prefix(separator)
            .ok_or_else(|| anyhow!("expected separator {separator:?}, found {orig:?}"))
    }

    pub fn try_chomp_prefix<S>(&mut self, separator: S) -> Option<&'a str>
    where
        S: AsRef<str>,
    {
        let s = self.0.get();
        if let Some((prefix, s)) = s.split_once(separator.as_ref()) {
            self.0.set(s);
            Some(prefix)
        } else {
            None
        }
    }

    pub fn require_prefix<S, P>(&mut self, separator: S, prefix: P) -> Result<()>
    where
        S: AsRef<str>,
        P: AsRef<str>,
    {
        let prefix = prefix.as_ref();
        let orig = self.0.get();
        let found = self.chomp_prefix(separator)?;
        if found == prefix {
            Ok(())
        } else {
            bail!("expected prefix {prefix:?}, found {found:?}, in {orig:?}")
        }
    }

    pub fn check_prefix<S, P>(&mut self, separator: S, prefix: P) -> bool
    where
        S: AsRef<str>,
        P: AsRef<str>,
    {
        self.require_prefix(separator, prefix).is_ok()
    }
}

impl<'a> From<&'a str> for Chomper<'a> {
    fn from(s: &'a str) -> Self {
        Chomper(Cell::new(s))
    }
}

impl<'a> From<Chomper<'a>> for &'a str {
    fn from(ch: Chomper<'a>) -> Self {
        ch.0.into_inner()
    }
}

impl<'a> AsRef<str> for Chomper<'a> {
    fn as_ref(&self) -> &str {
        self.0.get()
    }
}

impl<'a> AsRef<[u8]> for Chomper<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0.get().as_ref()
    }
}
