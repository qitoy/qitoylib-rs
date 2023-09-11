use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, LitStr};

#[proc_macro_attribute]
pub fn verify(attr: TokenStream, item: TokenStream) -> TokenStream {
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
        fn test(#[files(#path)] path: PathBuf) -> anyhow::Result<()> {
            #ident::verify(path)
        }
    }.into()
}
