#![feature(proc_macro_span)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, LitStr};

#[proc_macro_attribute]
pub fn verify(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = proc_macro::Span::call_site().source_file().path();
    let fnname: proc_macro2::TokenStream = path
        .file_stem()
        .and_then(|v| v.to_str())
        .unwrap_or("test")
        .parse()
        .unwrap();
    let attr = parse_macro_input!(attr as LitStr);
    let path = format!("./testcases/{}/*", attr.value());
    let imp = parse_macro_input!(item as ItemImpl);
    let syn::Type::Path(ty) = imp.self_ty.as_ref() else {
        return syn::Error::new_spanned(imp.self_ty, "expected `Path`")
            .to_compile_error()
            .into();
    };
    let ident = ty.path.get_ident().unwrap();
    quote! {
        #imp
        use std::path::PathBuf;
        #[rstest::rstest]
        fn #fnname(#[files(#path)] path: PathBuf) -> anyhow::Result<()> {
            #ident::verify(path)
        }
    }.into()
}
