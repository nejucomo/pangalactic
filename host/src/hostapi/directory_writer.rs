use crate::State;
use pangalactic_dagio::{DagioLink, HostDirectoryFor};
use pangalactic_handle::Handle;
use pangalactic_hostdir::HostDirectory;
use pangalactic_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn open<S>(
    mut caller: Caller<'_, State<S>>,
) -> Result<Handle<HostDirectoryFor<S>>, Trap>
where
    S: Store,
{
    Ok(caller
        .data_mut()
        .directory_writers_mut()
        .insert(HostDirectory::default()))
}

pub(super) async fn insert<S>(
    mut caller: Caller<'_, State<S>>,
    h_dir: Handle<HostDirectoryFor<S>>,
    nameptr: usize,
    namelen: usize,
    link: Handle<DagioLink<S>>,
) -> Result<(), Trap>
where
    S: Store,
{
    use crate::CallerIO;

    let namebytes = caller.guest_bytes_to_vec(nameptr, namelen)?;
    let name = String::from_utf8(namebytes).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    let link = caller.data().links().lookup(link)?.clone();
    let dir = caller
        .data_mut()
        .directory_writers_mut()
        .lookup_mut(h_dir)?;
    dir.insert(name, link)?;

    Ok(())
}

pub(super) async fn commit<S>(
    mut caller: Caller<'_, State<S>>,
    h_dir: Handle<HostDirectoryFor<S>>,
) -> Result<Handle<DagioLink<S>>, Trap>
where
    S: Store,
{
    let dir = caller.data_mut().directory_writers_mut().remove(h_dir)?;
    let link = caller.data_mut().dagio_mut().commit(dir).await?;
    let h_link = caller.data_mut().links_mut().insert(link);
    Ok(h_link)
}
