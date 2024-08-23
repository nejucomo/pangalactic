use pangalactic_link::Link;
use pangalactic_name::Name;
use serde::de::DeserializeOwned;
use std::collections::VecDeque;

pub(crate) fn parse_parts<C>(s: &str) -> anyhow::Result<(Link<C>, Vec<Name>)>
where
    C: DeserializeOwned,
{
    let mut q: VecDeque<&str> = s.split('/').collect();
    let linktext = q
        .pop_front()
        .ok_or_else(|| anyhow::anyhow!("missing link"))?;
    let link: Link<C> = linktext.parse()?;
    let parts = q.into_iter().map(|s| s.to_string()).collect();
    Ok((link, parts))
}
