mod adapter;

use self::adapter::HostFuncAdapter;
use crate::{
    FromGuestValue, HostFn0, HostFn1, HostFn2, HostFn3, HostFn4, HostFunc, IntoGuestReturn,
};
use std::fmt;
use wasmi::{Error, FuncRef, ModuleImportResolver, RuntimeArgs, RuntimeValue, Signature, Trap};

pub struct HostFuncResolver<V>(Vec<Entry<V>>);

struct Entry<V> {
    hf: Box<dyn HostFuncAdapter<V>>,
    funcref: FuncRef,
}

impl<V> HostFuncResolver<V>
where
    V: 'static,
{
    pub fn new() -> HostFuncResolver<V> {
        HostFuncResolver(vec![])
    }

    pub fn add_host_fn0<F, R, E>(&mut self, f: F)
    where
        F: Fn(&mut V) -> Result<R, E> + 'static,
        R: IntoGuestReturn + 'static,
        E: 'static,
        Trap: From<E>,
    {
        self.add_host_func(HostFn0::from(f))
    }

    pub fn add_host_fn1<F, A, R, E>(&mut self, f: F)
    where
        F: Fn(&mut V, A) -> Result<R, E> + 'static,
        A: FromGuestValue + 'static,
        R: IntoGuestReturn + 'static,
        E: 'static,
        Trap: From<E>,
    {
        self.add_host_func(HostFn1::from(f))
    }

    pub fn add_host_fn2<F, A1, A2, R, E>(&mut self, f: F)
    where
        F: Fn(&mut V, A1, A2) -> Result<R, E> + 'static,
        A1: FromGuestValue + 'static,
        A2: FromGuestValue + 'static,
        R: IntoGuestReturn + 'static,
        E: 'static,
        Trap: From<E>,
    {
        self.add_host_func(HostFn2::from(f))
    }

    pub fn add_host_fn3<F, A1, A2, A3, R, E>(&mut self, f: F)
    where
        F: Fn(&mut V, A1, A2, A3) -> Result<R, E> + 'static,
        A1: FromGuestValue + 'static,
        A2: FromGuestValue + 'static,
        A3: FromGuestValue + 'static,
        R: IntoGuestReturn + 'static,
        E: 'static,
        Trap: From<E>,
    {
        self.add_host_func(HostFn3::from(f))
    }

    pub fn add_host_fn4<F, A1, A2, A3, A4, R, E>(&mut self, f: F)
    where
        F: Fn(&mut V, A1, A2, A3, A4) -> Result<R, E> + 'static,
        A1: FromGuestValue + 'static,
        A2: FromGuestValue + 'static,
        A3: FromGuestValue + 'static,
        A4: FromGuestValue + 'static,
        R: IntoGuestReturn + 'static,
        E: 'static,
        Trap: From<E>,
    {
        self.add_host_func(HostFn4::from(f))
    }

    pub fn add_host_func<H>(&mut self, hostfunc: H)
    where
        H: HostFunc<V> + 'static,
    {
        // BUG? Why is 'static necessary since hostfunc moves into Entry?
        use wasmi::FuncInstance;

        let funcref = FuncInstance::alloc_host(hostfunc.signature(), self.0.len());
        let hf = Box::new(hostfunc);

        self.0.push(Entry { hf, funcref });
    }

    pub fn invoke_index(
        &self,
        vm: &mut V,
        index: usize,
        args: RuntimeArgs<'_>,
    ) -> Result<Option<RuntimeValue>, Trap> {
        use wasmi::TrapKind::TableAccessOutOfBounds;

        let entry = self.0.get(index).ok_or(Trap::new(TableAccessOutOfBounds))?;
        entry.hf.invoke(vm, args)
    }
}

impl<V> ModuleImportResolver for HostFuncResolver<V> {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        for entry in self.0.iter() {
            if entry.hf.name() == field_name {
                let providedsig = entry.funcref.signature();
                if providedsig == signature {
                    return Ok(entry.funcref.clone());
                } else {
                    return Err(Error::Instantiation(format!(
                        "Export {} signature mismatch: requested {:?}; provided {:?}",
                        field_name, signature, providedsig,
                    )));
                }
            }
        }

        return Err(Error::Instantiation(format!(
            "Export {} not found",
            field_name
        )));
    }
}

impl<V> fmt::Debug for HostFuncResolver<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbs = f.debug_struct("HostFuncResolver<...>");
        for entry in self.0.iter() {
            dbs.field(
                &entry.hf.name(),
                &format!("arity {}", entry.hf.signature().params().len()),
            );
        }
        dbs.finish()
    }
}
