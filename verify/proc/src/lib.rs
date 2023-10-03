use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse_macro_input, token, Ident, LitStr, Token};

#[derive(Debug)]
struct VerifyItem {
    verify: Punctuated<Verify, Token![,]>,
}

#[derive(Debug)]
struct Verify {
    ident: Ident,
    _brace_token: token::Paren,
    path: LitStr,
}

impl Parse for VerifyItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(VerifyItem {
            verify: input.parse_terminated(Verify::parse, Token![,])?,
        })
    }
}

impl Parse for Verify {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path;
        Ok(Verify {
            ident: input.parse()?,
            _brace_token: parenthesized!(path in input),
            path: path.parse()?,
        })
    }
}

/// verify! {}
///
/// # Example
///
/// ```
/// use verify::Verify;
///
/// struct Example;
///
/// impl Verify for Example {
///     fn solve(input: &str, stdout: &mut String) {
///         use proconio::{input, source::once::OnceSource};
///         use std::fmt::Write;
///         let input = OnceSource::from(input);
///         input! {
///             from input,
///             s: i32,
///         }
///         writeln!(stdout, "{}:{}:{}", s / 3600, s / 60 % 60, s % 60).unwrap();
///     }
/// }
///
/// verify! {
///     Example("aoj/ITP1_1_D"),
/// }
/// ```
#[proc_macro]
pub fn verify(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as VerifyItem);
    let mut verify = vec![];
    for Verify { ident, path, .. } in item.verify {
        let path = format!("./testcases/{}/*", path.value());
        let name: TokenStream2 = to_snake_case(ident.to_string()).parse().unwrap();
        verify.push(quote! {
            #[rstest::rstest]
            fn #name(#[files(#path)] path: PathBuf) -> anyhow::Result<()> {
                #ident::verify(path)
            }
        });
    }
    quote! {
        use std::path::PathBuf;
        #(#verify)*
    }
    .into()
}

fn to_snake_case(name: String) -> String {
    name.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() {
                let c = c.to_lowercase().next().unwrap();
                if i == 0 {
                    vec![c]
                } else {
                    vec!['_', c]
                }
            } else {
                vec![c]
            }
        })
        .collect()
}
