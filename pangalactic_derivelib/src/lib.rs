pub(crate) mod bindings;
mod bufreader;
mod bufwriter;
mod dirwriter;
mod link;
pub(crate) mod prim;

pub use self::bufreader::BufReaderHandle;
pub use self::bufwriter::BufWriterHandle;
pub use self::dirwriter::DirWriterHandle;
pub use self::link::{Kind, LinkHandle};

pub type PrimLinkHandle = prim::LinkHandle;

pub fn call_derive_impl<F>(exec: PrimLinkHandle, input: PrimLinkHandle, f: F) -> PrimLinkHandle
where
    F: FnOnce(LinkHandle, LinkHandle) -> LinkHandle,
{
    f(LinkHandle::from(exec), LinkHandle::from(input)).unwrap_prim()
}

#[macro_export]
macro_rules! wrap_derive {
    ( $impl:expr ) => {
        #[no_mangle]
        pub extern "C" fn derive(exec: PrimLinkHandle, input: PrimLinkHandle) -> PrimLinkHandle {
            $crate::call_derive_impl(exec, input, $impl)
        }
    };
}
