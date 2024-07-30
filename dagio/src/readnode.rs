use async_trait::async_trait;
use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::Store;

use crate::{Dagio, DagioLoad, DagioReader};

pub enum DagioReadNode<S>
where
    S: Store,
{
    FileReader(DagioReader<S>),
    Dir(HostDirectory<S::CID>),
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for DagioReadNode<S>
where
    S: Store,
{
    async fn load_from_dagio(
        dagio: &Dagio<S>,
        link: &Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Self> {
        use DagioReadNode::*;

        match link.kind() {
            LinkKind::File => {
                let r = dagio.load(link).await?;
                Ok(FileReader(r))
            }
            LinkKind::Dir => {
                let d = dagio.load(link).await?;
                Ok(Dir(d))
            }
        }
    }
}
