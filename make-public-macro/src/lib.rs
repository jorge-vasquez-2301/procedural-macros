use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Colon, Comma},
    Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident,
    Result, Variant, Visibility,
};

enum ItemType {
    NamedStruct,
    TupleStruct,
    Enum,
}

#[derive(Debug)]
enum Elements {
    Fields(Punctuated<Field, Comma>),
    Variants(Punctuated<Variant, Comma>),
}

#[derive(Debug)]
struct StructField {
    name: Option<Ident>,
    ty: Ident,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        match n {
            Some(n) => quote!(pub #n: #t),
            None => quote!(pub #t),
        }
        .to_tokens(tokens)
    }
}

impl Parse for StructField {
    fn parse(input: ParseStream) -> Result<Self> {
        let _vis: Visibility = input.parse()?;
        let ident = input.parse()?;
        let colon: bool = input.peek(syn::token::Colon);
        Ok(match colon {
            true => {
                let _colon: Colon = input.parse()?;
                let ty = input.parse()?;
                StructField {
                    name: Some(ident),
                    ty,
                }
            }
            false => StructField {
                name: None,
                ty: ident,
            },
        })
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    // eprintln!("{:#?}", &ast);

    let name = ast.ident;
    let attributes = ast.attrs;
    let item_type;

    let elements = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            item_type = ItemType::NamedStruct;
            Elements::Fields(named.clone())
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            item_type = ItemType::TupleStruct;
            Elements::Fields(unnamed.clone())
        }
        Data::Enum(DataEnum { variants, .. }) => {
            item_type = ItemType::Enum;
            Elements::Variants(variants)
        }
        _ => unimplemented!("Only works for structs/enums"),
    };

    let public_version = match elements {
        Elements::Fields(fields) => {
            let builder_fields = fields
                .iter()
                .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());

            match item_type {
                ItemType::NamedStruct => quote! {
                    #(#attributes)*
                    pub struct #name {
                        #(#builder_fields,)*
                    }
                },
                ItemType::TupleStruct => quote! {
                    #(#attributes)*
                    pub struct #name(
                        #(#builder_fields,)*
                    );
                },
                ItemType::Enum => unreachable!(),
            }
        }
        Elements::Variants(variants) => {
            let variants = variants.iter();
            quote! {
                #(#attributes)*
                pub enum #name {
                    #(#variants,)*
                }
            }
        }
    };

    public_version.into()
}
