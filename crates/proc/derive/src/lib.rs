use proc_macro::TokenStream;
use syn::parse::Parser;

mod query_readable;

/// query_readable!()
///
/// ```
/// # use qitoy_derive::query_readable;
/// # use proconio::input;
/// # use proconio::marker::Usize1;
/// // query
/// // * 0
/// // * 1 n s
/// // * 2 l r
/// query_readable!(Query, [
///     {},
///     { n: usize, s: String },
///     { l: Usize1, r: usize, },
/// ]);
///
/// # let source = proconio::source::auto::AutoSource::from("0 1 4 hoge 2 3 5");
/// // input
/// // 0
/// // 1 4 hoge
/// // 2 3 5
/// input! {
/// #   from source,
///     q0: Query,
///     q1: Query,
///     q2: Query,
/// }
///
/// assert!(matches!(q0, Query::Query0 {}));
/// assert!(matches!(q1, Query::Query1 { n, s } if n == 4 && s == "hoge".to_string()));
/// assert!(matches!(q2, Query::Query2 { l, r } if l == 2 && r == 5));
/// ```
#[proc_macro]
pub fn query_readable(input: TokenStream) -> TokenStream {
    query_readable::main.parse2(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
