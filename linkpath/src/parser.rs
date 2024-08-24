use std::collections::VecDeque;

use pangalactic_link::Link;
use pangalactic_name::{Name, NameError};
use serde::de::DeserializeOwned;

pub(crate) fn parse_parts<C>(s: &str) -> anyhow::Result<(Link<C>, Vec<Name>)>
where
    C: DeserializeOwned,
{
    let mut q: VecDeque<&str> = s.split('/').collect();
    let linktext = q
        .pop_front()
        .ok_or_else(|| anyhow::anyhow!("missing link"))?;
    let link: Link<C> = linktext.parse()?;
    let parts = q
        .into_iter()
        .map(Name::try_from)
        .collect::<Result<_, NameError>>()?;
    Ok((link, parts))
}
