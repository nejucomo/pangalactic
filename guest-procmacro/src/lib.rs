use proc_macro::TokenStream as PmTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn define_derive(attrs: PmTokenStream, body: PmTokenStream) -> PmTokenStream {
    define_derive_inner(attrs.into(), body.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

fn define_derive_inner(_attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    const DERIVE_IMPL: &str = "derive_impl";

    let guestfn: syn::ItemFn = syn::parse2(body)?;
    let gfspan = guestfn.span();

    let name = guestfn.sig.ident.to_string();
    if name != DERIVE_IMPL {
        return Err(syn::Error::new(
            gfspan,
            format!("Guest entrypoints must be named {DERIVE_IMPL:?}, not {name:?}."),
        ));
    }

    Ok(quote! {
        #[no_mangle]
        pub extern "C" fn prim_derive_impl(primplan: ::dagwasm_guest::prim::HandleLink) -> ::dagwasm_guest::prim::HandleLink {
            let plan = unsafe { ::dagwasm_guest::Link::wrap_handle(primplan) };
            let output = Link::from(derive_impl(plan.into()));
            unsafe { output.unwrap_handle() }
        }

        #guestfn
    })
}
