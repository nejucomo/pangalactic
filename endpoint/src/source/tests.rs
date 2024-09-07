use std::path::PathBuf;

use test_case::test_case;

use super::SourceEndpoint;

#[test_case("-", ())]
#[test_case(".", PathBuf::from("."))]
fn parse<T>(input: &str, expected: T)
where
    SourceEndpoint<()>: From<T>,
{
    let expected = SourceEndpoint::from(expected);
    let actual = input.parse().unwrap();
    assert_eq!(expected, actual);
}
