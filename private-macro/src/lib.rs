use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse_macro_input, DeriveInput};
use syn::{DataStruct, FieldsNamed, Ident};

struct ParsedField<'a> {
    name: &'a syn::Ident,
    ty: &'a syn::Type,
}

impl<'a> TryFrom<&'a syn::Field> for ParsedField<'a> {
    type Error = &'static str;

    fn try_from(field: &'a syn::Field) -> Result<Self, Self::Error> {
        Ok(Self {
            name: field.ident.as_ref().ok_or("field must have a name")?,
            ty: &field.ty,
        })
    }
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;

    let parsed_fields = parse_fields(&ast);

    let names = parsed_fields
        .iter()
        .map(|field| field.name)
        .collect::<Vec<_>>();
    let fields = generated_fields(&parsed_fields);
    let methods = generated_methods(&parsed_fields);

    quote!(
        struct #name {
            #(#fields),*
        }

        impl #name {
            fn new(#(#fields),*) -> Self {
                Self {
                    #(#names),*
                }
            }

            #(#methods)*
        }
    )
    .into()
}

fn parse_fields(ast: &DeriveInput) -> Vec<ParsedField> {
    let named_fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only works for structs with named fields"),
    };

    named_fields.iter().map(|f| f.try_into().unwrap()).collect()
}

fn generated_fields(parsed_fields: &[ParsedField]) -> Vec<proc_macro2::TokenStream> {
    parsed_fields
        .iter()
        .map(|&ParsedField { name, ty }| {
            quote!(
                #name: #ty
            )
        })
        .collect()
}

fn generated_methods(parsed_fields: &[ParsedField]) -> Vec<proc_macro2::TokenStream> {
    parsed_fields
        .iter()
        .map(|&ParsedField { name, ty }| {
            let method_name = Ident::new(&format!("get_{name}"), proc_macro2::Span::call_site());

            quote!(
                pub fn #method_name(&self) -> &#ty {
                    &self.#name
                }
            )
        })
        .collect()
}
