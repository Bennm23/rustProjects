use proc_macro2::TokenStream as TokenStream2;
use syn::{parse::Parse, Pat};
use quote::{quote, ToTokens};

pub struct Composition {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl Parse for Composition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}

impl quote::ToTokens for Composition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            sequence,
            conditions,
        } = &self.for_if_clause;

        tokens.extend(quote! {
            core::iter::IntoIterator::into_iter(#sequence).filter_map(
                move |#pattern| {
                    (true #(&& (#conditions))*).then(|| #mapping)
                }
            )
        });
    }
}

struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct ForIfClause {
    pattern: Pat,
    sequence: syn::Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![for]>()?;
        let pattern: Pat = Pat::parse_single(input)?;
        _ = input.parse::<syn::Token![in]>()?;
        let sequence: syn::Expr = input.parse()?;
        let conditions = parse_zero_or_more(input);

        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

fn parse_zero_or_more<T: Parse>(input: syn::parse::ParseStream) -> Vec<T> {
    let mut items = vec![];

    while let Ok(item) = input.parse() {
        items.push(item);
    }
    items
}


struct Condition(syn::Expr);

impl Parse for Condition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![if]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

