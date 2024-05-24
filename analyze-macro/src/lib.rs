use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::braced;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::token::Colon;
use syn::Attribute;
use syn::Ident;
use syn::Token;
use syn::Type;

#[derive(Debug)]
struct StructWithComments {
    ident: Ident,
    field_name: Ident,
    field_type: Type,
    outer_attributes: Vec<Attribute>,
    inner_attributes: Vec<Attribute>,
}

impl ToTokens for StructWithComments {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let debug = format!("{self:?}");

        let stream = quote! {
            #debug
        };

        tokens.extend(stream);
    }
}

impl Parse for StructWithComments {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let outer_attributes = input.call(Attribute::parse_outer).unwrap();
        let _: Token![struct] = input.parse().unwrap();
        let ident: Ident = input.parse().unwrap();

        let content;
        let _ = braced!(content in input);
        let inner_attributes = content.call(Attribute::parse_inner).unwrap();
        let field_name: Ident = content.parse().unwrap();
        let _: Colon = content.parse().unwrap();
        let field_type: Type = content.parse().unwrap();

        Ok(StructWithComments {
            ident,
            field_name,
            field_type,
            outer_attributes,
            inner_attributes,
        })
    }
}

#[proc_macro]
pub fn analyze(item: TokenStream) -> TokenStream {
    let result: StructWithComments = parse_macro_input!(item);
    quote! {
        #result
    }
    .into()
}
