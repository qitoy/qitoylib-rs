use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, token, Ident, Token};

struct Value {
    ident: Ident,
    _colon: Token![:],
    ty: Ident,
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _colon: input.parse()?,
            ty: input.parse()?,
        })
    }
}

struct Query {
    _brace: token::Brace,
    values: Punctuated<Value, Token![,]>,
}

impl Parse for Query {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let values;
        Ok(Self {
            _brace: braced!(values in input),
            values: values.parse_terminated(Value::parse, Token![,])?,
        })
    }
}

struct QueryEnum {
    ident: Ident,
    _brace: token::Brace,
    querys: Punctuated<Query, Token![,]>,
}

impl Parse for QueryEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let querys;
        Ok(Self {
            ident: input.parse()?,
            _brace: braced!(querys in input),
            querys: querys.parse_terminated(Query::parse, Token![,])?,
        })
    }
}

pub fn main(input: ParseStream) -> syn::Result<TokenStream> {
    let item = QueryEnum::parse(input)?;
    let qenum = make_enum(&item);
    let qimpl = make_impl(&item);
    Ok(quote! {
        #qenum
        #qimpl
    })
}

fn make_enum(item: &QueryEnum) -> TokenStream {
    let ident = &item.ident;
    let items: Vec<_> = item
        .querys
        .iter()
        .enumerate()
        .map(|(i, query)| {
            let ident = format_ident!("{}{}", ident, i);
            let field: Vec<_> = query
                .values
                .iter()
                .map(|Value { ident, ty, .. }| {
                    quote! { #ident: <#ty as ::proconio::source::Readable>::Output, }
                })
                .collect();
            quote! {
                #ident { #( #field )* },
            }
        })
        .collect();
    quote! {
        enum #ident {
            #( #items )*
        }
    }
}

fn make_impl(item: &QueryEnum) -> TokenStream {
    let ident = &item.ident;
    let arms: Vec<_> = item
        .querys
        .iter()
        .enumerate()
        .map(|(i, query)| {
            let ident = format_ident!("{}{}", ident, i);
            let items: Vec<_> = query
                .values
                .iter()
                .map(|Value { ident, ty, .. }| {
                    quote! {
                        #ident: #ty::read(source),
                    }
                })
                .collect();
            quote! { #i => Self::#ident { #( #items )* }, }
        })
        .collect();
    quote! {
        impl ::proconio::source::Readable for #ident {
            type Output = Self;
            fn read<R: ::std::io::BufRead, S: ::proconio::source::Source<R>>(
                source: &mut S
            ) -> Self::Output {
                match usize::read(source) {
                    #( #arms )*
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::parse::Parser;
    use pretty_assertions::assert_eq;
    use quote::quote;

    #[test]
    fn test1() {
        assert_eq!(
            main.parse2(quote! {
                Query {
                    {},
                    { a: Usize1, b: usize, },
                    { hoge: i32, fuga: Chars },
                }
            })
            .unwrap()
            .to_string(),
            quote! {
                enum Query {
                    Query0 {},
                    Query1 {
                        a: <Usize1 as ::proconio::source::Readable>::Output,
                        b: <usize as ::proconio::source::Readable>::Output,
                    },
                    Query2 {
                        hoge: <i32 as ::proconio::source::Readable>::Output,
                        fuga: <Chars as ::proconio::source::Readable>::Output,
                    },
                }

                impl ::proconio::source::Readable for Query {
                    type Output = Self;
                    fn read<R: ::std::io::BufRead, S: ::proconio::source::Source<R>>(
                        source: &mut S
                    ) -> Self::Output {
                        match usize::read(source) {
                            0usize => Self::Query0 {},
                            1usize => Self::Query1 {
                                a: Usize1::read(source),
                                b: usize::read(source),
                            },
                            2usize => Self::Query2 {
                                hoge: i32::read(source),
                                fuga: Chars::read(source),
                            },
                            _ => unreachable!(),
                        }
                    }
                }
            }
            .to_string(),
        );
    }
}
