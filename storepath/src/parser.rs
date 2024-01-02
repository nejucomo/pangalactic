use pangalactic_dir::Name;
use pangalactic_link::Link;
use pangalactic_store::Store;
use std::collections::VecDeque;

pub(crate) fn parse_parts<S>(s: &str) -> anyhow::Result<(Link<S>, Vec<Name>)>
where
    S: Store,
{
    let mut q: VecDeque<&str> = s.split('/').collect();
    let linktext = q
        .pop_front()
        .ok_or_else(|| anyhow::anyhow!("missing link"))?;
    let link: Link<S> = linktext.parse()?;
    let parts = q.into_iter().map(|s| s.to_string()).collect();
    Ok((link, parts))
}
