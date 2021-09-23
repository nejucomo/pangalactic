pub(crate) mod bindings;
mod link;
mod prim;

pub use self::link::{LinkHandle, LinkKind};
pub use self::prim::LinkPrim;

pub fn call_derive_impl<F>(exec: LinkPrim, input: LinkPrim, f: F) -> LinkPrim
where
    F: FnOnce(LinkHandle, LinkHandle) -> LinkHandle,
{
    f(LinkHandle::from(exec), LinkHandle::from(input)).unwrap_prim()
}

#[macro_export]
macro_rules! wrap_derive {
    ( $impl:expr ) => {
        #[no_mangle]
        pub extern "C" fn derive(exec: LinkPrim, input: LinkPrim) -> LinkPrim {
            $crate::call_derive_impl(exec, input, $impl)
        }
    };
}
