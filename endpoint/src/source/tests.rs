use std::path::PathBuf;

use test_case::test_case;

use crate::{
    Endpoint::{MkHos, MkStdio},
    HostOrStore::MkHost,
    HostPath, OriginEndpoint, Stdio,
};

#[test_case("-", MkStdio(Stdio))]
#[test_case(".", MkHos(MkHost(HostPath::from(PathBuf::from(".")))))]
fn parse(input: &str, expected: OriginEndpoint<()>) {
    let actual = input.parse().unwrap();
    assert_eq!(expected, actual);
}
