use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::Result;
use pangalactic_hash::Hash;
use pangalactic_linkpath::{LinkDestination, LinkPath};
use test_case::test_case;

use crate::{DestinationEndpoint, Endpoint, HostOrStore, HostPath, OriginEndpoint, Stdio};

type TestOrigin = OriginEndpoint<Hash>;
type TestDestination = DestinationEndpoint<Hash>;

#[test_case("-", Stdio.into_origin())]
#[test_case("-", Stdio.into_destination())]
#[test_case(".", ".".into_origin())]
#[test_case(".", ".".into_destination())]
#[test_case("foo/bar", "foo/bar".into_origin())]
#[test_case("foo/bar", "foo/bar".into_destination())]
#[test_case("/quz/wux", "/quz/wux".into_origin())]
#[test_case("/quz/wux", "/quz/wux".into_destination())]
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
        Endpoint::MkStdio(self)
    }
}

impl<'a> IntoOrigin for &'a str {
    fn into_origin(self) -> TestOrigin {
        HostOrStore::MkHost(self.parse().unwrap()).into_origin()
    }
}

impl IntoOrigin for HostOrStore<HostPath, LinkPath<Hash>> {
    fn into_origin(self) -> TestOrigin {
        Endpoint::MkHos(self)
    }
}

trait IntoDestination {
    fn into_destination(self) -> TestDestination;
}

impl IntoDestination for () {
    fn into_destination(self) -> TestDestination {
        HostOrStore::MkStore(LinkDestination::new_bare()).into_destination()
    }
}

impl IntoDestination for Stdio {
    fn into_destination(self) -> TestDestination {
        Endpoint::MkStdio(self)
    }
}

impl<'a> IntoDestination for &'a str {
    fn into_destination(self) -> TestDestination {
        HostOrStore::MkHost(self.parse().unwrap()).into_destination()
    }
}

impl IntoDestination for HostOrStore<HostPath, LinkDestination<Hash>> {
    fn into_destination(self) -> TestDestination {
        Endpoint::MkHos(self)
    }
}
