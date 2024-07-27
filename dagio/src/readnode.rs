use async_trait::async_trait;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::Store;

use crate::{Dagio, DagioHostDirectory, DagioLink, DagioLoad, DagioReader};

pub enum DagioReadNode<S>
where
    S: Store,
{
    FileReader(DagioReader<S>),
    Dir(DagioHostDirectory<S>),
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for DagioReadNode<S>
where
    S: Store,
{
    async fn load_from_dagio(dagio: &Dagio<S>, link: &DagioLink<S>) -> anyhow::Result<Self> {
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
