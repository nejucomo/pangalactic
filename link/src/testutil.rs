use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FakeKey(String);

pub fn fakekey() -> FakeKey {
    FakeKey("fake-key".to_string())
}
