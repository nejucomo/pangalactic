use crate::State;
use dagwasm_dagio::{DirectoryFor, LinkFor};
use dagwasm_dir::Directory;
use dagwasm_handle::Handle;
use dagwasm_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn open<S>(
    mut caller: Caller<'_, State<S>>,
) -> Result<Handle<DirectoryFor<S>>, Trap>
where
    S: Store,
{
    Ok(caller
        .data_mut()
        .directory_writers_mut()
        .insert(Directory::default()))
}

pub(super) async fn insert<S>(
    mut caller: Caller<'_, State<S>>,
    h_dir: Handle<DirectoryFor<S>>,
    nameptr: usize,
    namelen: usize,
    link: Handle<LinkFor<S>>,
) -> Result<(), Trap>
where
    S: Store,
{
    let namebytes = {
        let mut buf = vec![0; namelen]; // FIXME: don't allocate on guest-provided `len`.
        let mem = super::get_memory(&mut caller)?;
        buf.copy_from_slice(&mem.data(&caller)[nameptr..nameptr + namelen]);
        buf
    };

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
    h_dir: Handle<DirectoryFor<S>>,
) -> Result<Handle<LinkFor<S>>, Trap>
where
    S: Store,
{
    let dir = caller.data_mut().directory_writers_mut().remove(h_dir)?;
    let link = caller.data_mut().dagio_mut().commit(dir).await?;
    let h_link = caller.data_mut().links_mut().insert(link);
    Ok(h_link)
}
