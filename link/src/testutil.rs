use pangalactic_store::StoreCid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FakeKey(String);

pub fn fakekey() -> FakeKey {
    FakeKey("fake-key".to_string())
}

impl StoreCid for FakeKey {
    const SCHEME: &'static str = "test-fake";
}
