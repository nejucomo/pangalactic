use pangalactic_cid::ContentIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct FakeKey;

impl ContentIdentifier for FakeKey {}
