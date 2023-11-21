use proc_macro::TokenStream;

mod verify_docs;

#[proc_macro]
pub fn verify_docs(_item: TokenStream) -> TokenStream {
    verify_docs::main()
}
