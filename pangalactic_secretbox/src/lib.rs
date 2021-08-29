mod secretbox;

#[cfg(test)]
mod tests;

pub use secretbox::{OpenError, SecretBoxKey};
