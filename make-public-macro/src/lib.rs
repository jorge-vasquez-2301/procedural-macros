use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    token::Pub,
    Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, MetaList, Token,
    Visibility,
};

const EXCLUDE_ATTRIBUTE_NAME: &str = "exclude";

struct ExcludedFields {
    fields: Vec<String>,
}

impl ExcludedFields {
    fn matches_ident(&self, name: &Option<Ident>) -> bool {
        name.as_ref()
            .map(|n| n.to_string())
            .map(|n| self.fields.iter().any(|f| *f == n))
            .unwrap_or_else(|| false)
    }
}

impl Parse for ExcludedFields {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        match input.parse::<MetaList>() {
            Ok(meta_list) => {
                if meta_list
                    .path
                    .segments
                    .iter()
                    .find(|s| s.ident == EXCLUDE_ATTRIBUTE_NAME)
                    .is_some()
                {
                    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
                    let identifiers = parser.parse(meta_list.clone().tokens.into()).unwrap();
                    let fields = identifiers.iter().map(|v| v.to_string()).collect();
                    Ok(ExcludedFields { fields })
                } else {
                    Ok(ExcludedFields { fields: vec![] })
                }
            }
            Err(_) => Ok(ExcludedFields { fields: vec![] }),
        }
    }
}

#[proc_macro_attribute]
pub fn public(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    ast.vis = Visibility::Public(Pub(ast.span()));
    // eprintln!("{:#?}", &attr);
    let excluded_fields = parse_macro_input!(attr as ExcludedFields);

    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref mut named, .. }),
            ..
        }) => named.iter_mut().for_each(|field| {
            if !excluded_fields.matches_ident(&field.ident) {
                field.vis = Visibility::Public(Pub(field.span()))
            }
        }),
        Data::Struct(DataStruct {
            fields:
                Fields::Unnamed(FieldsUnnamed {
                    ref mut unnamed, ..
                }),
            ..
        }) => unnamed
            .iter_mut()
            .for_each(|field| field.vis = Visibility::Public(Pub(field.span()))),
        Data::Enum(..) => (),
        _ => unimplemented!("Only works for structs/enums"),
    };

    ast.to_token_stream().into()
}
