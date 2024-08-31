use std::path::PathBuf;

use test_case::test_case;

use super::SourceEndpoint::{self, Host, Stdin};

#[test_case("-" => Ok(Stdin))]
#[test_case("." => Ok(Host(PathBuf::from("."))))]
fn parse(input: &str) -> Result<SourceEndpoint<()>, String> {
    input.parse().map_err(|e| format!("{e}"))
}
