use crate::State;
use dagwasm_store::Store;
use wasmtime::{Caller, Memory};

pub(crate) trait CallerIO<S>
where
    S: Store,
{
    fn peek_into_guest<F, R>(&mut self, ptr: usize, len: usize, f: F) -> anyhow::Result<R>
    where
        F: FnOnce(&[u8]) -> R;
    fn guest_bytes_to_vec(&mut self, ptr: usize, len: usize) -> anyhow::Result<Vec<u8>>;
    fn write_into_guest(&mut self, dst: usize, src: &[u8]) -> anyhow::Result<()>;
}

impl<'a, S> CallerIO<S> for Caller<'a, State<S>>
where
    S: Store,
{
    fn peek_into_guest<F, R>(&mut self, ptr: usize, len: usize, f: F) -> anyhow::Result<R>
    where
        F: FnOnce(&[u8]) -> R,
    {
        let mem = get_memory(self)?;
        Ok(f(&mem.data(&self)[ptr..ptr + len]))
    }

    fn guest_bytes_to_vec(&mut self, ptr: usize, len: usize) -> anyhow::Result<Vec<u8>> {
        self.peek_into_guest(ptr, len, |slice| Vec::from(slice))
    }

    fn write_into_guest(&mut self, dst: usize, src: &[u8]) -> anyhow::Result<()> {
        let mem = get_memory(self)?;
        mem.data_mut(self)[dst..dst + src.len()].copy_from_slice(src);
        Ok(())
    }
}

fn get_memory<S>(caller: &mut Caller<'_, State<S>>) -> anyhow::Result<Memory>
where
    S: Store,
{
    use wasmtime::Extern::*;

    let export = caller
        .get_export("memory")
        .ok_or_else(|| anyhow::Error::msg("no 'memory' export found"))?;

    match export {
        Memory(m) => Ok(m),
        _ => Err(anyhow::Error::msg(
            "the 'memory' export is not a wasmtime::Memory",
        )),
    }
}
