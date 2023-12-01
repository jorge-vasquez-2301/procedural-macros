use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse_macro_input, spanned::Spanned, token::Pub, Data, DataStruct, DeriveInput, Fields,
    FieldsNamed, FieldsUnnamed, Visibility,
};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    ast.vis = Visibility::Public(Pub(ast.span()));
    // eprintln!("{:#?}", &ast);

    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref mut named, .. }),
            ..
        }) => named
            .iter_mut()
            .for_each(|field| field.vis = Visibility::Public(Pub(field.span()))),
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
