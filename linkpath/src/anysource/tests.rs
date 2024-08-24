use std::path::PathBuf;

use test_case::test_case;

use super::AnySource::{self, Host, Stdin};

#[test_case("-" => Ok(Stdin))]
#[test_case("." => Ok(Host(PathBuf::from("."))))]
fn parse(input: &str) -> Result<AnySource<()>, String> {
    input.parse().map_err(|e| format!("{e}"))
}
