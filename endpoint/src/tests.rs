use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::Result;
use pangalactic_hash::Hash;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_linkpath::{LinkDestination, LinkPath};
use test_case::test_case;

use crate::{
    DestinationEndpoint,
    Endpoint::{MkHos, MkStdio},
    HostOrStore::{self, MkHost, MkStore},
    HostPath, OriginEndpoint, Stdio,
};

type TestOrigin = OriginEndpoint<Hash>;
type TestDestination = DestinationEndpoint<Hash>;

#[test]
#[ignore]
fn dump_linkpath() {
    let lp = get_linkpath();
    panic!("This test fails simply to display this linkpath:\n{lp}");
}

fn get_linkpath() -> LinkPath<Hash> {
    let link = Link::new(LinkKind::Dir, Hash::of(""));
    LinkPath::new(link, "a/dir/with/a/file.txt").unwrap()
}

#[test_case("-", Stdio.into_origin())]
#[test_case("-", Stdio.into_destination())]
#[test_case(".", ".".into_origin())]
#[test_case(".", ".".into_destination())]
#[test_case("foo/bar", "foo/bar".into_origin())]
#[test_case("foo/bar", "foo/bar".into_destination())]
#[test_case("/quz/wux", "/quz/wux".into_origin())]
#[test_case("/quz/wux", "/quz/wux".into_destination())]
#[test_case(
    "pg:D:rxNJufX5oaagQE3qNtzJSZvLJcmtwRK3zJqTyuQfMmI/a/dir/with/a/file.txt",
    get_linkpath().into_origin()
)]
#[test_case(
    "pg:D:rxNJufX5oaagQE3qNtzJSZvLJcmtwRK3zJqTyuQfMmI/a/dir/with/a/file.txt",
    get_linkpath().into_destination()
)]
#[test_case("pg:", ().into_destination())]
fn parse_and_display<T>(text: &str, value: T) -> Result<()>
where
    T: Display + Debug + FromStr<Err = anyhow::Error> + PartialEq,
{
    let actualval = text.parse()?;
    assert_eq!(value, actualval);
    let textout = actualval.to_string();
    assert_eq!(text, &textout);
    let textout2 = value.to_string();
    assert_eq!(text, &textout2);
    Ok(())
}

trait IntoOrigin {
    fn into_origin(self) -> TestOrigin;
}

impl IntoOrigin for Stdio {
    fn into_origin(self) -> TestOrigin {
        MkStdio(self)
    }
}

impl<'a> IntoOrigin for &'a str {
    fn into_origin(self) -> TestOrigin {
        MkHost(self.parse().unwrap()).into_origin()
    }
}

impl IntoOrigin for HostOrStore<HostPath, LinkPath<Hash>> {
    fn into_origin(self) -> TestOrigin {
        MkHos(self)
    }
}

impl IntoOrigin for LinkPath<Hash> {
    fn into_origin(self) -> TestOrigin {
        MkStore(self).into_origin()
    }
}

trait IntoDestination {
    fn into_destination(self) -> TestDestination;
}

impl IntoDestination for () {
    fn into_destination(self) -> TestDestination {
        MkStore(LinkDestination::new_bare()).into_destination()
    }
}

impl IntoDestination for Stdio {
    fn into_destination(self) -> TestDestination {
        MkStdio(self)
    }
}

impl<'a> IntoDestination for &'a str {
    fn into_destination(self) -> TestDestination {
        MkHost(self.parse().unwrap()).into_destination()
    }
}

impl IntoDestination for HostOrStore<HostPath, LinkDestination<Hash>> {
    fn into_destination(self) -> TestDestination {
        MkHos(self)
    }
}

impl IntoDestination for LinkPath<Hash> {
    fn into_destination(self) -> TestDestination {
        LinkDestination::<Hash>::try_from(self)
            .unwrap()
            .into_destination()
    }
}

impl IntoDestination for LinkDestination<Hash> {
    fn into_destination(self) -> TestDestination {
        MkStore(self).into_destination()
    }
}
