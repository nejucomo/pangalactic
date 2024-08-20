use anyhow::Result;
use pangalactic_layer_dir::LinkDirectory;
use pangalactic_link::Link;

pub(crate) fn trace_insert<C>(
    prefixpath: &str,
    d: &mut LinkDirectory<C>,
    name: &str,
    link: Link<C>,
) -> Result<()>
where
    C: serde::Serialize,
{
    tracing::debug!("committing \"{}/{}\" -> {}", prefixpath, name, &link);
    d.insert(name.to_string(), link)
}
