use pangalactic_dir::Name;
use pangalactic_link::Link;
use std::collections::VecDeque;

pub(crate) fn parse_parts<K>(s: &str) -> anyhow::Result<(Link<K>, Vec<Name>)>
where
    K: std::fmt::Debug + serde::de::DeserializeOwned,
{
    let mut q: VecDeque<&str> = s.split('/').collect();
    let linktext = q
        .pop_front()
        .ok_or_else(|| anyhow::anyhow!("missing link"))?;
    let link: Link<K> = linktext.parse()?;
    let parts = q.into_iter().map(|s| s.to_string()).collect();
    Ok((link, parts))
}
