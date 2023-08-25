use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Colon,
    Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Result, Type,
    Visibility,
};

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
    let is_named_struct;

    let fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            is_named_struct = true;
            named
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            is_named_struct = false;
            unnamed
        }
        _ => unimplemented!("Only works for structs"),
    };

    let builder_fields = fields
        .iter()
        .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());

    let public_version = if is_named_struct {
        quote! {
            pub struct #name {
                #(#builder_fields,)*
            }
        }
    } else {
        quote! {
            pub struct #name(
                #(#builder_fields,)*
            );
        }
    };

    public_version.into()
}
